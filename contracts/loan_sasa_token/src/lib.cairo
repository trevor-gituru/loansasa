use core::starknet::ClassHash;
use starknet::ContractAddress;

// State changing public functions
#[starknet::interface]
pub trait ILoanSasaTokenState<TContractState> {
    fn approve(ref self: TContractState, borrower: ContractAddress, amount: u256);
    fn buyTokens(ref self: TContractState, amount: u256);
    fn mint(ref self: TContractState, amount: u256);
    fn transfer(ref self: TContractState, reciepient: ContractAddress, amount: u256);
    fn transferFrom(ref self: TContractState, from: ContractAddress, amount: u256);
    fn upgrade(ref self: TContractState, new_class_hash: ClassHash);


}
// Viewer public functions
#[starknet::interface]
pub trait ILoanSasaTokenView<TContractState> {
    fn allowance(self: @TContractState, lender: ContractAddress,
        borrower: ContractAddress) -> u256;
    fn balanceOf(self: @TContractState, account: ContractAddress) -> u256;
    fn canMint(self: @TContractState) -> bool;
    fn decimals(self: @TContractState) -> u8;
    fn get_owner(self: @TContractState) -> ContractAddress;
    fn name(self: @TContractState) -> felt252;
    fn symbol(self: @TContractState) -> felt252;
    fn totalSupply(self: @TContractState) -> u256;

}


#[starknet::contract]
mod LoanSasaToken {
    use starknet::event::EventEmitter;
use core::num::traits::Zero;
    use core::starknet::{
        ContractAddress, ClassHash,
        class_hash::class_hash_const,
        contract_address::contract_address_const,
        get_caller_address, get_contract_address,
        syscalls, SyscallResultTrait
    };
    use openzeppelin::token::erc20::interface::{IERC20, 
        ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
    use openzeppelin::token::erc20::dual20::DualCaseERC20;
    use starknet::storage::{
        StoragePointerReadAccess, StoragePointerWriteAccess,
        StoragePathEntry, Map
    };
    use super::{ILoanSasaTokenState, ILoanSasaTokenView};
    

    const DECIMALS: u8 = 18;
    const ETH_LST_RATE: u256 = 1000;
    const NAME: felt252 = 'LoanSasaToken';
    const SYMBOL: felt252 = 'LST';

    #[storage]
    struct Storage {
        account_balances: Map<ContractAddress, u256>,
        approvals: Map<ContractAddress, Map<ContractAddress, u256>>,
        owner: ContractAddress,
        total_supply: u256
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner_account: ContractAddress) {
        self.owner.write(owner_account);
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        Approval: Approval,
        Mint: Mint,
        Transfer: Transfer,
        Upgrade: Upgrade,

    }

    #[derive(Drop, starknet::Event)]
    struct Approval {
        #[key]
        lender: ContractAddress,
        #[key]
        borrower: ContractAddress,
        amount: u256
    }
    /// @dev Represents a mint action successfuly performed
    #[derive(Drop, starknet::Event)]
    struct Mint {
        #[key]
        account: ContractAddress,
        amount: u256
    }

    /// @dev Represents a transfer action successfuly performed
    #[derive(Drop, starknet::Event)]
    struct Transfer {
        #[key]
        from: ContractAddress,
        #[key]
        to: ContractAddress,
        amount: u256
    }

    /// @dev Represents a transfer action successfuly performed
    #[derive(Drop, starknet::Event)]
    struct Upgrade {
        by: ContractAddress,
    }

    #[abi(embed_v0)]
    impl LoanSasaTokenStateImpl of super::ILoanSasaTokenState<ContractState> {
        fn approve(ref self: ContractState, borrower: ContractAddress, amount: u256){
            let lender: ContractAddress = (get_caller_address());
            assert!(self.balanceOf(lender) >= amount, "LENDER HAS INSUFFICIENT BALANCE");
            self.approvals.entry(lender).entry(borrower).write(amount);
            self.emit(Approval{lender, borrower, amount});

        }

        fn buyTokens(ref self: ContractState, amount: u256){
            let buyer: ContractAddress = (get_caller_address());
            let contract_account: ContractAddress = (get_contract_address());
            let eth_address: ContractAddress = contract_address_const::<
                0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7
            >();
            let eth_dispatcher = ERC20ABIDispatcher {
                contract_address: eth_address
            };

            // Transfer the ETH to the caller
            eth_dispatcher
            .transfer_from(
                buyer,
                contract_account,
                amount
            );
            let lst_bought: u256 = amount * ETH_LST_RATE;

            let mut call_data: Array<felt252> = array![];
            Serde::serialize(@buyer, ref call_data);
            Serde::serialize(@lst_bought, ref call_data);

            syscalls::call_contract_syscall(
                contract_account, selector!("transfer"), call_data.span()
            ).unwrap_syscall();
        }
        
        fn mint(ref self: ContractState, amount: u256){
            let account: ContractAddress = (get_caller_address());
            assert!(self._isOwner(account), "UNAUTHORIZED ACCOUNT");
            assert!(self.canMint(), "FREE TOKENS ARE ABOVE THRESHOLD");
            
            let contract_account: ContractAddress = (get_contract_address());
            let new_free_token: u256 = self.account_balances.entry(contract_account).read() + amount;
            let new_supply: u256 = self.total_supply.read() + amount; 
            self.total_supply.write(new_supply);
            self.account_balances.entry(contract_account).write(new_free_token);
            self.emit(Mint{account, amount});
        }

        fn transfer(ref self: ContractState, reciepient: ContractAddress, amount: u256){
            let sender: ContractAddress = (get_caller_address());

            assert!(self._sufficientBalance(sender, amount), "INSUFFICIENT BALANCE");
            let new_balance: u256 = self.account_balances.entry(sender).read() - amount;
            self.account_balances.entry(sender).write(new_balance);
            let new_balance: u256 = self.account_balances.entry(reciepient).read() + amount;
            self.account_balances.entry(reciepient).write(new_balance);
            self.emit(Transfer{
                from: sender, 
                to: reciepient, amount});
        }

        fn transferFrom(ref self: ContractState, from: ContractAddress, amount: u256){
            let to: ContractAddress = (get_caller_address());
            let approval_amount: u256 = self.allowance(from, to);
            assert!(amount <= approval_amount, "INSUFFICIENT APPROVAL AMOUNT");
            assert!(self._sufficientBalance(from, amount), "INSUFFICIENT BALANCE");

            let new_balance: u256 = self.account_balances.entry(from).read() - amount;
            self.account_balances.entry(from).write(new_balance);
            let new_balance: u256 = self.account_balances.entry(to).read() + amount;
            self.account_balances.entry(to).write(new_balance);
            let new_balance: u256 = approval_amount - amount;
            self.approvals.entry(from).entry(to).write(new_balance);
            self.emit(Transfer{from, to, amount});
        }

        fn upgrade(ref self: ContractState, new_class_hash: ClassHash) {
            let caller: ContractAddress = get_caller_address();
            assert!(self._isOwner(caller), "INSUFFICIENT AUTHORITY");
            assert!(!(new_class_hash.is_zero()), "Class hash cannot be zero");
            syscalls::replace_class_syscall(new_class_hash).unwrap();
            self.emit(Upgrade{by: caller});
        }

    }

    #[abi(embed_v0)]
    impl LoanSasaTokenViewImpl of super::ILoanSasaTokenView<ContractState> {
        fn allowance(self: @ContractState, lender: ContractAddress,
                borrower: ContractAddress) -> u256{
            self.approvals.entry(lender).entry(borrower).read()
        }
        fn balanceOf(self: @ContractState, account: ContractAddress) -> u256 {
            self.account_balances.entry(account).read()
        }

        fn canMint(self: @ContractState) -> bool {
            let threshold: u256 = self.total_supply.read() / 4;
            let contract_address: ContractAddress = get_contract_address();
            let free_tokens: u256 = self.account_balances.entry(contract_address).read();
            (free_tokens <= threshold)
        }

        fn decimals(self: @ContractState) -> u8 {
            (DECIMALS)
        }

        fn get_owner(self: @ContractState) -> ContractAddress {
            (self.owner.read())
        }

        fn name(self: @ContractState) -> felt252 {
            (NAME)
        }

        fn symbol(self: @ContractState) -> felt252 {
            (SYMBOL)
        }

        fn totalSupply(self: @ContractState) -> u256 {
            self.total_supply.read()
        }
    }

    #[generate_trait]
    impl InternalUserFunctions of InternalUserFunctionsTraits{
        fn _isOwner(self: @ContractState, account: ContractAddress) -> bool{
            let owner: ContractAddress = self.owner.read();
            (owner == account)
        }
        fn _sufficientBalance(self: @ContractState, account: ContractAddress,
            transfer_amount: u256
        ) -> bool{
            let balance: u256 = self.balanceOf(account);
            (transfer_amount < balance)
        }

    }
    
}

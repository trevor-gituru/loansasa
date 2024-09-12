use starknet::ContractAddress;

// Viewer public functions
#[starknet::interface]
pub trait ILoanSasaTokenView<TContractState> {
    fn balanceOf(self: @TContractState, account: ContractAddress) -> u256;
    fn canMint(self: @TContractState) -> bool;
    fn decimals(self: @TContractState) -> u8;
    fn get_owner(self: @TContractState) -> ContractAddress;
    fn name(self: @TContractState) -> felt252;
    fn symbol(self: @TContractState) -> felt252;
    fn totalSupply(self: @TContractState) -> u256;  
}

// State changing public functions
#[starknet::interface]
pub trait ILoanSasaTokenState<TContractState> {
    fn mint(ref self: TContractState, amount: u256);
    fn transfer(ref self: TContractState, reciepient: ContractAddress, amount: u256);

}


#[starknet::contract]
mod LoanSasaToken {
    use core::starknet::{ContractAddress, get_caller_address,
        get_contract_address};
    use starknet::storage::{
        StoragePointerReadAccess, StoragePointerWriteAccess,
        StoragePathEntry, Map
    };
    use super::{ILoanSasaTokenState, ILoanSasaTokenView};
    
    const NAME: felt252 = 'LoanSasaToken';
    const SYMBOL: felt252 = 'LST';
    const DECIMALS: u8 = 18;

    #[storage]
    struct Storage {
        account_balances: Map<ContractAddress, u256>,
        owner: ContractAddress,
        totalSupply: u256
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner_account: ContractAddress) {
        self.owner.write(owner_account);
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        Mint: Mint,
        Transfer: Transfer,

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
    

    #[abi(embed_v0)]
    impl LoanSasaTokenStateImpl of super::ILoanSasaTokenState<ContractState> {
        fn mint(ref self: ContractState, amount: u256){
            let account: ContractAddress = (get_caller_address());
            assert!(self._isOwner(account), "UNAUTHORIZED ACCOUNT");
            assert!(self.canMint(), "FREE TOKENS ARE ABOVE THRESHOLD");
            
            let contract_account: ContractAddress = (get_contract_address());
            let prev_free_token: u256 = self.account_balances.entry(contract_account).read();
            let prev_supply: u256 = self.totalSupply.read(); 
            self.totalSupply.write(prev_supply + amount);
            self.account_balances.entry(contract_account).write(prev_free_token + amount);
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
        

    }

    #[abi(embed_v0)]
    impl LoanSasaTokenViewImpl of super::ILoanSasaTokenView<ContractState> {
        fn balanceOf(self: @ContractState, account: ContractAddress) -> u256 {
            self.account_balances.entry(account).read()
        }

        fn canMint(self: @ContractState) -> bool {
            let threshold: u256 = self.totalSupply.read() / 4;
            let contract_address: ContractAddress = get_contract_address();
            let free_tokens: u256 = self.account_balances.entry(contract_address).read();
            if free_tokens > threshold{
                return false;
            }
            return true;
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
            self.totalSupply.read()
        }
    }

    #[generate_trait]
    impl InternalUserFunctions of InternalUserFunctionsTraits{
        fn _isOwner(self: @ContractState, account: ContractAddress) -> bool{
            let owner: ContractAddress = self.owner.read();
            if owner == account {
                return true;
            }
            return false;
        }
        fn _sufficientBalance(self: @ContractState, account: ContractAddress,
            transfer_amount: u256
        ) -> bool{
            let balance: u256 = self.balanceOf(account);
            if transfer_amount > balance{
                return false;
            }else{
                return true;
            }
        }

    }
    
}

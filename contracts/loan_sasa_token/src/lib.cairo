use starknet::ContractAddress;

#[starknet::interface]
pub trait ILoanSasaToken<TContractState> {
    // Callers
    fn balanceOf(self: @TContractState, account: ContractAddress) -> u256;
    fn decimals(self: @TContractState) -> u8;
    fn get_owner(self: @TContractState) -> ContractAddress;
    fn name(self: @TContractState) -> felt252;
    fn symbol(self: @TContractState) -> felt252;
    fn totalSupply(self: @TContractState) -> u256;
    // Invokers
    

}

#[starknet::contract]
mod LoanSasaToken {
    use core::starknet::{ContractAddress, get_caller_address};
    use starknet::storage::{
        StoragePointerReadAccess, StoragePointerWriteAccess,
        StoragePathEntry, Map
    };
    use super::ILoanSasaToken;
    
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

    #[abi(embed_v0)]
    impl LoanSasaTokenImpl of super::ILoanSasaToken<ContractState> {
        // Callers
        fn balanceOf(self: @ContractState, account: ContractAddress) -> u256 {
            self.account_balances.entry(account).read()
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
}

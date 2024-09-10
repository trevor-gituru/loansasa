use starknet::ContractAddress;

#[starknet::interface]
pub trait ILoanSasaToken<TContractState> {
    // Getters
    fn get_owner(self: @TContractState) -> ContractAddress;
    fn name(self: @TContractState) -> felt252;
    //Setters
    fn increase_balance(ref self: TContractState, amount: felt252);
    fn add_user(ref self: TContractState, username: felt252);
    fn get_balance(self: @TContractState) -> felt252;
    fn get_user(self: @TContractState) -> felt252;
    

}

#[starknet::contract]
mod LoanSasaToken {
    use starknet::storage::{
        StoragePointerReadAccess, StoragePointerWriteAccess, StoragePathEntry, Map
    };
    use core::starknet::{ContractAddress, get_caller_address};


    #[storage]
    struct Storage {
        balance: felt252,
        users: Map<ContractAddress, felt252>,
        owner: ContractAddress 
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner_account: ContractAddress) {
        self.owner.write(owner_account);
    }

    #[abi(embed_v0)]
    impl LoanSasaTokenImpl of super::ILoanSasaToken<ContractState> {
        fn increase_balance(ref self: ContractState, amount: felt252) {
            assert(amount != 0, 'Amount cannot be 0');
            self.balance.write(self.balance.read() + amount);
        }

        fn get_balance(self: @ContractState) -> felt252 {
            self.balance.read()
        }
        
        fn add_user(ref self: ContractState, username: felt252) {
            let address = get_caller_address();
            self.users.entry(address).write(username);
        }

        fn get_user(self: @ContractState) -> felt252 {
            self.users.entry(get_caller_address()).read()
        }

        fn get_owner(self: @ContractState) -> ContractAddress {
            (self.owner.read())
        }

        fn name(self: @ContractState) -> felt252 {
            ("LoanSasaToken")
        }
    }
}

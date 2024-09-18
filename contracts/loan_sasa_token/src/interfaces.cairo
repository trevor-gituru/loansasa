use core::starknet::ClassHash;
use starknet::ContractAddress;
// State changing public functions
#[starknet::interface]
pub trait ILoanSasaTokenState<TContractState> {
    fn approve(ref self: TContractState, borrower: ContractAddress, amount: u256);
    fn buyTokens(ref self: TContractState, amount: u256);
    fn createPledge(ref self: TContractState, 
        amount: u256, period: u64);
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
    // fn fetchLoan(self: @TContractState, local_id: u64) -> Array<u128>;
    fn get_owner(self: @TContractState) -> ContractAddress;
    fn name(self: @TContractState) -> felt252;
    fn symbol(self: @TContractState) -> felt252;
    fn totalSupply(self: @TContractState) -> u256;
}

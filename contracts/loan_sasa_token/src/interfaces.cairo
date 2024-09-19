use core::starknet::ClassHash;
use loan_sasa_token::LoanSasaToken::ArrayData;
use starknet::ContractAddress;

// State changing public functions
#[starknet::interface]
pub trait ILoanSasaTokenState<TContractState> {
    fn approve(ref self: TContractState, borrower: ContractAddress, amount: u256);
    fn buyTokens(ref self: TContractState, amount: u256);
    fn createLoan(ref self: TContractState, 
        amount: u256, period: u64);
    fn deleteLoan(ref self: TContractState, loan_id: u64);
    fn mint(ref self: TContractState, amount: u256);
    fn payLoan(ref self: TContractState, loan_id: u64, amount: u256);
    fn reclaimLoan(ref self: TContractState, loan_id: u64);
    fn signLoan(ref self: TContractState, loan_id: u64);
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
    fn collaterals(self: @TContractState) -> u256;
    fn decimals(self: @TContractState) -> u8;
    fn fetchLoan(self: @TContractState, local_id: u64) -> Array<ArrayData>;
    fn filterLoan(self: @TContractState, amount: u256, period: u64) -> Array<u64>;
    fn get_owner(self: @TContractState) -> ContractAddress;
    fn interests(self: @TContractState, loan_id: u64) -> u256;
    fn name(self: @TContractState) -> felt252;
    fn pledges(self: @TContractState) -> u256;
    fn symbol(self: @TContractState) -> felt252;
    fn totalSupply(self: @TContractState) -> u256;
}

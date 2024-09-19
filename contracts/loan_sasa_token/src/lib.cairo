mod interfaces;

#[starknet::contract]
mod LoanSasaToken {
// -------------------------
//       Import Section
// -------------------------
// Import the data types, traits & fn needed for smart contract.
    use core::num::traits::Zero;
    use core::starknet::{
        ContractAddress, ClassHash,
        class_hash::class_hash_const,
        contract_address::contract_address_const,
        get_block_timestamp,
        get_caller_address, get_contract_address,
        syscalls, SyscallResultTrait,
    };
    use openzeppelin::token::erc20::interface::{IERC20, 
        ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
    use openzeppelin::token::erc20::dual20::DualCaseERC20;
    use starknet::event::EventEmitter;
    use starknet::storage::{
        StoragePointerReadAccess, StoragePointerWriteAccess,
        StoragePathEntry, Map,
        Vec, VecTrait, MutableVecTrait
    };
    use super::interfaces::{ILoanSasaTokenState, ILoanSasaTokenView};

// -------------------------
//       Constants Section
// -------------------------
// Define constant variables that are used throughout the contract.
    const COLLATERAL: u256 = 50;
    const DECIMALS: u8 = 18;
    const ETH_LST_RATE: u256 = 1_000;
    const MAX_PERIOD: u64 = 31_579_200; // Avergae time of a year
    const NAME: felt252 = 'LoanSasaToken';
    const SYMBOL: felt252 = 'LST';

// -------------------------
//       Data Types Section
// -------------------------
// Define custom data types used by the contract.
    #[derive(Copy, Drop, Serde, starknet::Store)]
    pub enum ArrayData {
        U8: u8,
        U64: u64,
        U256: u256,
        Felt: felt252,
        Address: ContractAddress,
    }
    #[derive(Copy, Drop, Serde, starknet::Store)]
    pub struct Loan {
        id: u64,
        lender: ContractAddress,
        borrower: Option<ContractAddress>,
        amount: u256,
        period: u64,
        signed_on: Option<u64>,
        status: LoanStatus
    }

    #[generate_trait]
    impl LoanImpl of LoanTrait {
        fn sign(ref self: Loan, borrower: ContractAddress){
            let time: u64 = get_block_timestamp();
            self.borrower = Option::Some(borrower);
            self.signed_on = Option::Some(time);
            self.status = LoanStatus::Active;

        }

        fn toArray(loan: Loan) -> Array<ArrayData>{
            let mut loan_arr:Array<ArrayData> = ArrayTrait::new();
            let borrower: ContractAddress = loan.borrower
                                .unwrap_or(contract_address_const::<0x0>());
            loan_arr.append(ArrayData::U64(loan.id));
            loan_arr.append(ArrayData::Address(loan.lender));
            loan_arr.append(ArrayData::Address(borrower));
            loan_arr.append(ArrayData::U256(loan.amount));
            loan_arr.append(ArrayData::U64(loan.period));
            loan_arr.append(ArrayData::U64(loan.signed_on.unwrap_or(0_u64)));
            loan_arr.append(ArrayData::U8(loan.status.to_u8()));
            return loan_arr;
        }

    }

    #[derive(Copy, Drop, Serde, starknet::Store)]
    pub enum LoanStatus {
        Pending,       // Loan has been offered but not yet accepted
        Active,        // Loan is active and has been accepted by the borrower
        Repaid,        // Loan has been fully repaid
        Defaulted,     // Loan has defaulted and collateral has been claimed
        Closed,        // Loan has been closed or terminated
    }
    #[generate_trait]
    impl LoanStatusImpl of LoanStatusTrait{
        fn to_u8(self: @LoanStatus) -> u8{
            match self {
                LoanStatus::Pending => { 0_u8 },
                LoanStatus::Active => { 1_u8 },
                LoanStatus::Repaid => { 2_u8},
                LoanStatus::Defaulted => { 3_u8 },
                LoanStatus::Closed => { 4_u8 },
            }
        }
    }
    
// -------------------------
//       Events Section
// -------------------------
// Define events that will be emitted during contract execution.
    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        Approval: Approval,
        Loans: LoanEvent,
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
    
    #[derive(Drop, starknet::Event)]
    struct LoanEvent {
        global_id: u64,
        local_id: u64,
        lender: ContractAddress,
        borrower: ContractAddress,
        amount: u256,
        period: u64,
        signed_on: u64,
        status: u8
    }
    #[generate_trait]
    impl LoanEventImpl of LoanEventTrait {
        fn new(loan: Loan, local_id: u64) -> LoanEvent{
            let borrower: ContractAddress = loan.borrower
                        .unwrap_or(contract_address_const::<0x0>());
            let signed_on: u64 = loan.signed_on.unwrap_or(0);
            LoanEvent {
                global_id: loan.id,
                local_id: local_id,
                lender: loan.lender,
                borrower: borrower,
                amount: loan.amount,
                period: loan.period,
                signed_on: signed_on,
                status: loan.status.to_u8()
            }
        }
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
// -------------------------
//       Storage Section
// -------------------------
// Define the storage variables here, which will be persistent across contract calls.
    #[storage]
    struct Storage {
        account_balances: Map<ContractAddress, u256>,
        approvals: Map<ContractAddress, Map<ContractAddress, u256>>,
        collaterals: u256,
        loans_counter: u64,
        loans: Vec<Option<Loan>>,
        owner: ContractAddress,
        pledges: u256,
        total_supply: u256
    }
// -------------------------
//       Constructor Section
// -------------------------
// Constructor to initialize contract state (owner and initial balance).
    #[constructor]
    fn constructor(ref self: ContractState, owner_account: ContractAddress) {
        self.owner.write(owner_account);
        self.loans_counter.write(0_u64);
    }
// -------------------------
//       Implementation Section
// -------------------------
// Implement the contract functions and logic.
     
    #[abi(embed_v0)]
    impl LoanSasaTokenStateImpl of ILoanSasaTokenState<ContractState> {
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
        
        fn createLoan(ref self: ContractState,
                amount: u256, period: u64){
            let lender: ContractAddress = (get_caller_address());
            assert!(self._sufficientBalance(lender, amount), "INSUFFICIENT BALANCE");
            assert!(period <= MAX_PERIOD, "PERIOD EXCEEDED A YEAR");
            self._transferToPledges(lender, amount);
            let loan_id: u64 = self.loans_counter.read();
            let loan: Loan = Loan {
                id: loan_id,
                lender,
                borrower: Option::None,
                amount,
                signed_on: Option::None,
                period,
                status: LoanStatus::Pending
            };
            let local_id: u64 = self._insertLoan(loan);
            let loan_event: LoanEvent = LoanEventImpl::new(loan, local_id);
            self.emit(loan_event);
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

        fn signLoan(ref self: ContractState, loan_id: u64) {
            let borrower: ContractAddress = get_caller_address();
            let mut loan: Loan = self._getLoan(loan_id);
            assert!(loan.status.to_u8() == 0, "LOAN IS ALREADY ACTIVE");
            let collateral: u256 = (loan.amount * COLLATERAL) / 100;
            assert!(self._sufficientBalance(borrower, collateral), "INSUFFICIENT BORROWER BALANCE");
            self._transferCollaterals(borrower, collateral);
            loan.sign(borrower);
            self._transferFromPledges(borrower, loan.amount);
            self.loans.at(loan_id).write(Option::Some(loan));
            let loan_event: LoanEvent = LoanEventImpl::new(loan, loan_id);
            self.emit(loan_event);
            
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
    impl LoanSasaTokenViewImpl of ILoanSasaTokenView<ContractState> {
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

        fn fetchLoan(self: @ContractState, local_id: u64) -> Array<ArrayData> {
            assert!(local_id < self.loans.len(), "INVALID LOAN ID");
            let loan: Option<Loan> = self.loans.at(local_id).read();
            if loan.is_none(){
                return ArrayTrait::<ArrayData>::new();
            }
            let loan_arr = LoanImpl::toArray(loan.unwrap());
            loan_arr
        }

        fn filterLoan(self: @ContractState, amount: u256, period: u64) -> Array<u64> {
            let mut matching_loans: Array<u64> = array![];
            let mut i: u64 = 0;
            while i < self.loans.len(){
                let current_loan: Option<Loan> = self.loans.at(i).read();
                if current_loan.is_some(){
                    let current_loan: Loan = current_loan.unwrap();
                    if (amount <= current_loan.amount) && 
                            (period <= current_loan.period) && 
                            (current_loan.status.to_u8() == 0) {
                        matching_loans.append(i);
                    }
                }
                i = i + 1;
            };
            return matching_loans;
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

        fn col(self: @ContractState) -> u256 {
            self.collaterals.read()
        }

        fn pl(self: @ContractState) -> u256 {
            self.pledges.read()
        }
    }

    #[generate_trait]
    impl InternalViewFunctions of InternalViewFunctionsTraits{
        fn _getLoan(self: @ContractState, loan_id: u64) -> Loan{
            assert!(loan_id < self.loans.len(), "INVALID LOAN ID");
            let loan: Option<Loan> = self.loans.at(loan_id).read();
            assert!(loan.is_some(), "LOAN DOES NOT EXIST");
            let loan: Loan = loan.unwrap();
            return loan;

        }

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
    #[generate_trait]
    impl InternalStateFunctions of InternalStateFunctionsTraits{
        fn _insertLoan(ref self: ContractState, new_loan: Loan) -> u64{
            // Check for an empty vec
            let next_global_id: u64 = self.loans_counter.read() + 1_u64;
            if self.loans.len() == 0{
                self.loans.append().write(Option::Some(new_loan));
                self.loans_counter.write(next_global_id);
                return 0_u64;
            }
            // Check for vec with empty slot
            let mut i: u64 = 0;
            let mut found_slot: bool = false;
            while i < self.loans.len(){
                let current_loan: Option<Loan> = self.loans.at(i).read();
                if current_loan.is_none(){
                    self.loans.at(i).write(Option::Some(new_loan));
                    found_slot = true;
                    break;
                }
                i = i + 1;
            };
            self.loans_counter.write(next_global_id);
            if found_slot{
                return i;
            }
            self.loans.append().write(Option::Some(new_loan));
            return (self.loans.len() - 1_u64);
        }



        fn _transferCollaterals(ref self: ContractState, account: ContractAddress, amount: u256){
            let new_balance = self.account_balances.entry(account).read() - amount;
            self.account_balances.entry(account).write(new_balance);
            let new_balance = self.collaterals.read() + amount;
            self.collaterals.write(new_balance);
        }

        fn _transferToPledges(ref self: ContractState, account: ContractAddress, amount: u256){
            let new_balance = self.account_balances.entry(account).read() - amount;
            self.account_balances.entry(account).write(new_balance);
            let new_balance = self.pledges.read() + amount;
            self.pledges.write(new_balance);
        }
        fn _transferFromPledges(ref self: ContractState, borrower: ContractAddress, amount: u256){
            let new_balance = self.pledges.read() - amount;
            self.pledges.write(new_balance);
            let new_balance = self.account_balances.entry(borrower).read() + amount;
            self.account_balances.entry(borrower).write(new_balance);
            
        }

    }
}

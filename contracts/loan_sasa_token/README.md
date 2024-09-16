# LoanSasaToken
## Brief Overview

The `LoanSasaToken` contract in Cairo is designed to manage tokens on StarkNet. It includes functionality to mint new tokens, transfer tokens between accounts, and allow delegated transfers based on pre-approved allowances. The contract maintains essential storage variables to track token supply, balances, and allowances.

## Key Concepts

- **Storage Management**: The contract uses storage variables to keep track of token supply, individual balances, and allowances, which are crucial for maintaining the state of the token contract.
  
- **Minting**: The `mint` function allows for the creation of new tokens, with proper access control typically required to prevent unauthorized minting.
  
- **Transfer and Approval Mechanism**: The `transfer` function moves tokens directly, while `transferFrom` allows for delegated transfers based on pre-approved allowances.

- **Security**: Security considerations are critical, including checks for sufficient balances, proper allowance handling, and access controls to prevent misuse.

- **Testing and Optimization**: This basic implementation serves as a demonstration. Production-ready contracts should be thoroughly tested, audited, and optimized to handle edge cases and ensure robustness.

## Tasks
### Section A - Users
#### 0. Rename Contract
- Rename all instances of `HelloStarknet` to `LoanSasaToken` and test it out as follows:
 ```bash
razaoul@razaoul-HP-ProBook-450-G4:~/loansasa/contracts/loan_sasa_token$: sed -i 's/HelloStarknet/LoanSasaToken/g' src/lib.cairo tests/test_contract.cairo
razaoul@razaoul-HP-ProBook-450-G4:~/loansasa/contracts/loan_sasa_token$: snforge test
   Compiling loan_sasa_token v0.1.0 (/home/razaoul/loansasa/contracts/loan_sasa_token/Scarb.toml)
    Finished release target(s) in 2 seconds


Collected 2 test(s) from loan_sasa_token package
Running 0 test(s) from src/
Running 2 test(s) from tests/
[PASS] tests::test_contract::test_cannot_increase_balance_with_zero_value (gas: ~104)
[PASS] tests::test_contract::test_increase_balance (gas: ~170)
Tests: 2 passed, 0 failed, 0 skipped, 0 ignored, 0 filtered out
``` 
#### 1. Add owner
- Add the `owner` variable to hold the account address of administrator of smart contract

#### 2. Initialize owner
- Create a constructor that takes in the `owner_account` and sets it to the `owner` storage

#### 3. Fetch owner
- Create a view function `get_owner` that fetches and returns the owner of the contract

#### 4. Account Balances
- Create the `account_balances` storage var to store the LST held by user accounts
- Create the `balanceOf` view function that returns the LST tokens held by the account address provided.

### Section B - Tokens
#### 0. Create name of token
- Create the `name` view function to return the name of the token, `LoanSasaToken`

#### 1. Create Symbol of toke
- The `symbol` view function will return the symbol of the token, `LST`

#### 2. Create totalSupply
- Create `totalSupply` storage variable which will hold total amount of LST tokens in existance
- Create `decimals` view function to return the number of decimals that represent 1 LST, which will be 18. This means that the smallest unit of **LST** is 1/10^18 of 1 LST.
- Create `totalSupply` view function which returns total LST in existance.

#### 3. Mint tokens
LST will be minted when free tokens fall below 25% of total supply and only owner can mint it.
- Create a `_isOwner` interal fn that returns boolean status of ownership
- Create a `canMint` view fn that returns a bool of whether more LST can be minted based upon aboce condition.
- Create a `Mint` event to record the amount & account who minted tokens
- Create a `mint` public state function that takes `amount` to be added:
    + Check if caller is owner else panics with `UNAUTHORIZED ACTION`
    + Check if threshold has been reached else panics with `FREE TOKENS ABOVE THRESHOLD`
    + Updates the `totalSupply` and free tokens (**Account for the contract**)
    + Emits a `Mint` event on success

#### 4. Transfer Tokens
LST will be able to transferred from the account holders to any receipient as long as they have sufficient funds
- Create a `_sufficientBalance` internal fn that checks whether a given account can withdraw a certain number of LST
- Create a `Transfer` event that contains (from, to, amount).
- Create a `transfer` state fn that:
    + Takes in `amount` to transfer
    + If account has `INSUFFICIENT BALANCE` a panic occurs
    + Else the amount is transfered to receipient's account.
    + Emit the `Transfer` event on success.

#### 5. Buy LST via ETH
Users will be able to buy LST currrenly only via ETH at an exchange rate of 1 ETH: 1000 LST
- Add `openzeppelin` dependecy to use the Disatchers to access ETH contract
- Create a `buyTokens` state fn that:
    + Takes in `amount` of ETH paid.
    + Checks if they have approved the contract to transfer above funds from etherium else panics with  `ACCOUNT HASNT APPROVED ETH TRANSFER`
    + Transfers the ETH tokens from client to contract account
    + Transfer the equivalent `LST` from contract account to client

#### 6. Add p2p money transfer
This is where one of the LST holder **A** approves **B** to be able to withdraw certain amount of LST at any given time. It assumes the borrower and lender have already created their own loan agreement & loan repayement is not settled on blockchain

##### I. Approval
- Add the `approval` storage var in form of `Map<lender, Map<borrower, amount>>`
- Create the `Approval` event that emits approval details
- Create the `approve` state fn where:
    + It takes in the `borrower` account & `amount` to lend
    + Check if `lender`has sufficient balance & if not panics
    + Adds the details to the `approval` storage var
    + Emits the `approval` event

##### II. Allowance
- Add the `allowance`view fn that:
    + Takes in the `lender`, `borrower` parameters
    + Returns the `amount` approved
     
##### III. TransferFrom
- Create the `transferFrom` state fn that:
    + Takes in `from`, `amount`.
    + Asserts that the `amount` has been approved & sufficient sender balance.
    + Carry out respective LST transfer
    + Emit `Transfer` event 

#### 7. Add loan repayement
The smart contract should create a mechanism that enables auto loan repayement, it will be as follows:
- The Lender will pledge a certain amount of LST that will be held by smart contract for loans and will specify max period of loan
- Any prospective borrower will query with blockchain available loan contracts they can take based on specified amount and period
- The can then sign the loan contract where they are expected to give a 115% collateral based on loan, after which loan is automatically transferred to their account
- The borrower can then pay only the full amount of loan before expected time
- The lender has the option of checking status of loan, if borrower still hasnt paid they have the option of withdrawing the collateral of offered by the borrower.
- Smart contract charges 3% handler fee & loan rate is principal plus 0.3% monthly interest

##### I. Lender pledge
- Create a `pledges` storage var to store overall pledges of lenders
- Create a `_transferPledges` internal fn to handle transfer of tokens betweens lenders & pledges
- Create a `loans_counter` that counts number of loan contracts created and initialized to 0 on creation smart contract
- Create a `Loan` structure that holds:
    + `id`: From `loans_counter`
    + `lender` address
    + `borrower` address
    + `amount` borrowed
    + `signed_on` - time `borrower` accepted loan
    + `period` - Expected loan repayment period
    + `status` of loan
- Create a `LoanStatus` enum that describes status of loan as follows:
    + `Pending`,       // Loan has been offered but not yet accepted
    + `Active`,        // Loan is active and has been accepted by the borrower
    + `Repaid`,        // Loan has been fully repaid
    + `Defaulted`,     // Loan has defaulted and collateral has been claimed
    + `Closed`,        // Loan has been closed or terminated
- Create a `LoanEvent` that emits the loan structure, the only difference being that it has a `local_id` which is Loans id in storage array
- Create a `loans` storage vec to hold loan contracts
- Create a `_insertLoan` internal fn that:
    + Accpets a `Loan` 
    + Loops through the vec to find an empty slot and if not appends the loan to a new slot
    + Returns the slot id
- Create a `createLoanPledge` state fn that:
    + Takes in `amount` and `period` (seconds) to pledge
    + Asserts that lender has sufficient balance & that period not exceed 1 year
    + Transfer the LST to `pledges`
    + Emit a 
### Section C - Contract
#### 0. Upgradability
- Create `Upgrade` event that has the caller of upgrade
- Create an `upgrade` state fn that:
    + Takes in new declared classHash & replaces the old one
    + Emit `Upgrade` event on success
## Resources
- [ERC 20](https://docs.openzeppelin.com/contracts/3.x/api/token/erc20#ERC20-name--)

How will i approve loans:
- Blockchain, just like buyingTokens p2p, add approve & transferFrom
approve:

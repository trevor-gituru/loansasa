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
### 0. Rename Contract
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
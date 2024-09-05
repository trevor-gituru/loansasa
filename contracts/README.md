# Tasks
## 0. Install Starknet-foundry
- It is a toolkit for developing Starknet contracts 
```bash
$ curl -L https://raw.githubusercontent.com/foundry-rs/starknet-foundry/master/scripts/install.sh | sh
$ snfoundryup -v 0.27.0
$ snforge --version
snforge 0.27.0
```
## 1. Install Scarb
- It is a package manager and build tool for the Cairo programming language
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh
$ scarb --version
scarb 2.7.1 (e288874ba 2024-08-13)
cairo: 2.7.1 (https://crates.io/crates/cairo-lang-compiler/2.7.1)
sierra: 1.6.0
```

## 2. Create LoanSasaToken smart contract
- To managing and issuing tokens within the LoanSasa ecosystem, create its smart contract
```bash
$ snforge init loan_sasa_token
✔ Which test runner do you want to set up? · Starknet Foundry (default)
    Updating git repository https://github.com/foundry-rs/starknet-foundry
Created `loan_sasa_token` package.
```

# Resources
- [starknet-foundry](https://github.com/foundry-rs/starknet-foundry)
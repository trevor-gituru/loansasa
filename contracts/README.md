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
## 2. Install starknet-devnet
- It is a local testnet for Starknet... in Rust!
- To install starknet-devnet with docker
```bash
$ docker pull shardlabs/starknet-devnet-rs
$ echo "alias starknet-devnet='docker run --network host shardlabs/starknet-devnet-rs'" >> ~/.bashrc
$ source ~/.bashrc
$ starknet-devnet --version
starknet-devnet 0.1.2
```
## 3. Create LoanSasaToken smart contract
- To managing and issuing tokens within the LoanSasa ecosystem, create its smart contract
```bash
$ snforge init loan_sasa_token
✔ Which test runner do you want to set up? · Starknet Foundry (default)
    Updating git repository https://github.com/foundry-rs/starknet-foundry
Created `loan_sasa_token` package.
```
- Follow README in `loan_sasa_token/README.md` to properly create smart contract

## 

# Resources
- [starknet-foundry](https://github.com/foundry-rs/starknet-foundry)
- [Smart Contract in Cairo](https://book.cairo-lang.org/ch01-01-installation.html)
- [Starknet-devnet-rs](https://github.com/0xSpaceShard/starknet-devnet-rs)

sncast --url http://127.0.0.1:5050 call -a 0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7 -f balanceOf -c 0x6ad01af0e0b75af392828b382b0f5c04ae5170d35aded779c2c6a60a758bc0a

sncast --url http://127.0.0.1:5050 --account t1 invoke -a 0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7 -f approve -v v1 -c 0x6ad01af0e0b75af392828b382b0f5c04ae5170d35aded779c2c6a60a758bc0a 100 0

sncast --url http://127.0.0.1:5050 call -a 0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7 -f allowance -c 0x302a349b229b085fe5fccaa2c54548458f87ddf66e2f0a3e007a8466eeed63a 0x6ad01af0e0b75af392828b382b0f5c04ae5170d35aded779c2c6a60a758bc0a

sncast --url http://127.0.0.1:5050 --account t2 invoke -a 0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7 -f transferFrom -v v1 -c 0x302a349b229b085fe5fccaa2c54548458f87ddf66e2f0a3e007a8466eeed63a 0x6ad01af0e0b75af392828b382b0f5c04ae5170d35aded779c2c6a60a758bc0a 1 0
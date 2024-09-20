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

## 4. Testing smart contract
- Run starknet-devnet to simulate starknet, predeploying 20 oz account, generating blocks after every 60sec & using below seed so that accounts will have fixed details on running starrknet-devnet again.
```bash
$ starknet-devnet --seed 20240825 --accounts 20 --block-generation-on 60
```
- Add 2 account profiles to sncast to use for interacting with devnet whose details can be found from accounts listed on running devnet
```bash
$ sncast --url http://127.0.0.1:5050 account add -n t1 -t oz -a 0x302a349b229b085fe5fccaa2c54548458f87ddf66e2f0a3e007a8466eeed63a --private-key 0x98af61ad9810dfd713706712f7d82a59 --public-key 0x6fbdf45ea83aeb11c0dcd628fd530308b1acd2d6a05429b6a07bad5b4c3d39a --class-hash 0x61dac032f228abef9c6626f995015233097ae253a7f72d68552db02f2971b8f --add-profile t1
command: account add
add_profile: Profile t1 successfully added to snfoundry.toml
$ sncast --url http://127.0.0.1:5050 account add -n t2 -t oz -a 0x6ad01af0e0b75af392828b382b0f5c04ae5170d35aded779c2c6a60a758bc0a --private-key 0xec9d3f676e1f28289e62468e0ce7f593 --public-key 0x113aa21cc2c733a5bcae4b0e4c4b8ddb9ab40ead578109280805ae39ad41dfe --class-hash 0x61dac032f228abef9c6626f995015233097ae253a7f72d68552db02f2971b8f --add-profile t2
command: account add
add_profile: Profile t2 successfully added to snfoundry.toml
```
- Declare the smart contract (from within its root repo):
```bash
$ sncast --url http://127.0.0.1:5050 --account t1 declare -v v3 -c LoanSasaToken
   Compiling loan_sasa_token v0.1.0 (/home/razaoul/Documents/rust_projetcs/loansasa/contracts/loan_sasa_token/Scarb.toml)
    Finished release target(s) in 41 seconds
command: declare
class_hash: 0x156ee2afe68161f10af4d0fb047c5c54211eaddbd754624e79600de0b5186ba
transaction_hash: 0x3aa008dfe7ef2d7ab510979826fb34dee1a187035af47abd9d3e0613e9acfcc
```
- Deploy smart contract with address of t1 account
```bash 
$ sncast --url http://127.0.0.1:5050 --account t1 deploy -v v3 -g 0x156ee2afe68161f10af4d0fb047c5c54211eaddbd754624e79600de0b5186ba -c 0x302a349b229b085fe5fccaa2c54548458f87ddf66e2f0a3e007a8466eeed63a
command: deploy
contract_address: 0x3f3e43c95c05c8e7784eb93a9ac0f27bddcd327371dd1e48a9698aa8ea50e69
transaction_hash: 0x19eba942db5839a353b2a86dabcbd6ff2002406e3ec6d673e26aca2737a0dac
```
- Mint new LST using owner of contract (t1)
```bash
$ sncast --url http://127.0.0.1:5050 --account t1 invoke -v v3 -a 0x3f3e43c95c05c8e7784eb93a9ac0f27bddcd327371dd1e48a9698aa8ea50e69 -f mint -c 1000000000 0
command: invoke
transaction_hash: 0x75c61808e429fb267c891c2c88ab6c966f48aae8660b893583e874e0ce150ab
```
- query total supply of tokens
```bash
$ sncast --url http://127.0.0.1:5050 call -a 0x3f3e43c95c05c8e7784eb93a9ac0f27bddcd327371dd1e48a9698aa8ea50e69 -f totalSupply
command: call
response: [0x3b9aca00, 0x0]
```
- To buy tokens one must first approve the contract to transfer certain amount of tokens from ETH as follows, where bought 10,000LST worth 10ETH (*NO DECIMALS*):
```bash
$ sncast --url http://127.0.0.1:5050 --account t1 invoke -v v3 -a 0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7 -f approve -c 0x3f3e43c95c05c8e7784eb93a9ac0f27bddcd327371dd1e48a9698aa8ea50e69 500000 0
command: invoke
transaction_hash: 0x316de8a307ba324cc7afd0be98daae8a7160f59d0586418c0e89a5f4e0e0f92
$ sncast --url http://127.0.0.1:5050 --account t1 invoke -v v3 -a 0x3f3e43c95c05c8e7784eb93a9ac0f27bddcd327371dd1e48a9698aa8ea50e69 -f buyTokens -c 10 0
command: invoke
transaction_hash: 0x4d073ca8fe8b9a7d560500726ec374eaa8a20e790d430adda4c434c091d2c78
$ sncast --url http://127.0.0.1:5050 call -a 0x3f3e43c95c05c8e7784eb93a9ac0f27bddcd327371dd1e48a9698aa8ea50e69 -f balanceOf -c 0x302a349b229b085fe5fccaa2c54548458f87ddf66e2f0a3e007a8466eeed63a
command: call
response: [0x2710, 0x0]
```
**NB//** For above `-v v3` specifies using strk tokens for gas fees

## 5. Testing JSON-RPC
- To test whether events are successfully generated one can use postman to make `POST` request based on contract details and block number
- One can import the `LoanSasaToken.postman_collection.json` to postman and change config like block number, contract address & event keys to better test events of smart contract

# Resources
- [starknet-foundry](https://github.com/foundry-rs/starknet-foundry)
- [Smart Contract in Cairo](https://book.cairo-lang.org/ch01-01-installation.html)
- [Starknet-devnet-rs](https://github.com/0xSpaceShard/starknet-devnet-rs)
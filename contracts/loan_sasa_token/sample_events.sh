#!/bin/bash
# Script to test fn invokers as well as its events
rpc='http://127.0.0.1:5050'
eth='0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7'
t1='0x302a349b229b085fe5fccaa2c54548458f87ddf66e2f0a3e007a8466eeed63a'
t2='0x6ad01af0e0b75af392828b382b0f5c04ae5170d35aded779c2c6a60a758bc0a'
class_hash="$(sncast --url $rpc --account t1 declare -v v3 -c LoanSasaToken | grep "class_hash" | cut -d ' ' -f 2)"
contract_address="$(sncast --url $rpc --account t1 deploy -v v3 -g $class_hash -c $t1 | grep "contract_address" | cut -d ' ' -f 2)"


# Approve token transfer
sncast --url $rpc --account t1 invoke -v v3 -a $eth -f approve -c $contract_address 5000000 0
sncast --url $rpc --account t2 invoke -v v3 -a $eth -f approve -c $contract_address 5000000 0
# Mint tokens
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f mint -c 5000000 5000000
# Buy 50,000 LST
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f buyTokens -c 50 0
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f buyTokens -c 50 0
# Transfer 10,000 LST
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f transfer -c $t2 10000 0
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f transfer -c $t1 10000 0
# Create 1000 loan to be paid in 10 minutes
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f createLoan -c 1000 0 600
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f createLoan -c 1000 0 600
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f createLoan -c 1000 0 600
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f createLoan -c 1000 0 600
## Loan to be paid in 1 seconds
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f createLoan -c 1000 0 1
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f createLoan -c 1000 0 1
# Agree to loans
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f signLoan -c 1
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f signLoan -c 0
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f signLoan -c 4
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f signLoan -c 5
# Pay loans
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f payLoan -c 1 1000 0
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f payLoan -c 0 1000 0

# Delete loans
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f deleteLoan -c 2
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f deleteLoan -c 3

# Reclaim loans
sleep 60
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f reclaimLoan -c 4
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f reclaimLoan -c 5
# Test approve
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f approve -c $t2 5000 0
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f approve -c $t1 5000 0
# Test transferFrom
sncast --url $rpc --account t1 invoke -v v3 -a $contract_address -f transferFrom -c $t2 5000 0
sncast --url $rpc --account t2 invoke -v v3 -a $contract_address -f transferFrom -c $t1 5000 0

echo $contract_address

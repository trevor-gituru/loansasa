// use starknet::{ContractAddress, contract_address_const, get_caller_address};

// use snforge_std::{declare, ContractClassTrait, 
//     start_cheat_account_contract_address_global, 
//     stop_cheat_account_contract_address_global};

// use loan_sasa_token::ILoanSasaTokenSafeDispatcher;
// use loan_sasa_token::ILoanSasaTokenSafeDispatcherTrait;
// use loan_sasa_token::ILoanSasaTokenDispatcher;
// use loan_sasa_token::ILoanSasaTokenDispatcherTrait;

// fn deploy_contract(name: ByteArray) -> ContractAddress {
//     let contract = declare(name).unwrap();
//     let (contract_address, _) = contract.deploy(@ArrayTrait::new()).unwrap();
//     contract_address
// }

// #[test]
// fn test_increase_balance() {
//     let contract_address = deploy_contract("LoanSasaToken");

//     let dispatcher = ILoanSasaTokenDispatcher { contract_address };

//     let balance_before = dispatcher.get_balance();
//     assert(balance_before == 0, 'Invalid balance');

//     dispatcher.increase_balance(42);

//     let balance_after = dispatcher.get_balance();
//     assert(balance_after == 42, 'Invalid balance');
// }

// #[test]
// #[feature("safe_dispatcher")]
// fn test_cannot_increase_balance_with_zero_value() {
//     let contract_address = deploy_contract("LoanSasaToken");

//     let safe_dispatcher = ILoanSasaTokenSafeDispatcher { contract_address };

//     let balance_before = safe_dispatcher.get_balance().unwrap();
//     assert(balance_before == 0, 'Invalid balance');

//     match safe_dispatcher.increase_balance(0) {
//         Result::Ok(_) => core::panic_with_felt252('Should have panicked'),
//         Result::Err(panic_data) => {
//             assert(*panic_data.at(0) == 'Amount cannot be 0', *panic_data.at(0));
//         }
//     };
// }

// #[test]
// fn test_add_user() {
//     let contract_address = deploy_contract("LoanSasaToken");

//     let dispatcher = ILoanSasaTokenDispatcher { contract_address };
//     let cheat_addr: ContractAddress = 0x302a349b229b085fe5fccaa2c54548458f87ddf66e2f0a3e007a8466eeed63a.try_into().unwrap();
//     start_cheat_account_contract_address_global(cheat_addr);
//     // Change the caller address to 123 when calling the contract at the `contract_address` address
//     dispatcher.add_user('ken');
//     println!("account_address {}", dispatcher.get_address());
//     let added_user = dispatcher.get_user();
//     assert!(added_user == 'ken', "User not found");
// }

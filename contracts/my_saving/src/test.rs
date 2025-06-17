#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, Vec};

fn client<'a>(env: &'a Env, contract_id: &Address) -> ContractClient<'a> {
    ContractClient::new(env, contract_id)
}

#[test]
fn test_deposite() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = client(&env, &contract_id);

    let user_addr = Address::generate(&env);
    let amount = 10000u64;
    client.initialize(&user_addr);
    client.deposite(&user_addr, &amount);
    env.as_contract(&contract_id, || {
        //subjective but here re-registring the env for comparaison
        let instance = env.storage().instance();
        let dp_list: Vec<DepositeEvent> =
            instance.get(&DataKeys::Deposite).unwrap_or(Vec::new(&env));
        assert_eq!(dp_list.len(), 1);

        let dp = dp_list.first().unwrap();
        assert_eq!(dp.amount, amount);
        assert_eq!(dp.user_addr, user_addr);
    });
}

#[test]
fn test_withdrawal() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = client(&env, &contract_id);
    let user_addr = Address::generate(&env);
    let owner = Address::generate(&env);
    let initial_deposit: u64 = 100;
    let withdrawal_amount: u64 = 40;

    client.initialize(&owner);
    client.deposite(&user_addr, &initial_deposit);
    client.widthdrawal(&user_addr, &withdrawal_amount);
    let new_balance = client.get_balance(&user_addr);
    assert_eq!(
        new_balance,
        i64::try_from(initial_deposit - withdrawal_amount).expect("Sign error")
    );

    let events = client.get_all_widthdrawal(&owner); // Fixed typo here too
    assert_eq!(events.len(), 1);
    let wd_event = events.get(0).unwrap();
    assert_eq!(wd_event.amount, withdrawal_amount);
    assert_eq!(wd_event.user_addr, user_addr);
}

#[test]
fn test_acc() {}

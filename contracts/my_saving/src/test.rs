#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn client<'b>(env: &'b Env) -> ContractClient<'b> {
    let contract_id = env.register(Contract, ());
    ContractClient::new(&env, &contract_id)
}

#[test]
fn test_deposite() {
    let env = Env::default();
    let client = client(&env);
    let instance = env.storage().instance();

    let user_addr = Address::generate(&env);
    let amount = 10000;
    env.as_contract(&user_addr, || {
        client.deposite(&user_addr, &amount);

        let dp_list: Vec<DepositeEvent> = instance
            .get(&DataKeys::Deposite)
            .expect("expect deposites list");
        let dp = dp_list.first().unwrap();

        assert_eq!(dp_list.len(), 1);
        assert_eq!(dp.amount, amount);
        assert_eq!(dp.user_addr, user_addr);
    });
    // let users
    //assert_eq!(dp_list.len(), 1);
}

#[test]
fn test_widthdrawal() {}

#[test]
fn test_acc() {}

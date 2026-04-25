#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address};

#[test]
fn test_happy_path_payment() {
    let env = Env::default();
    let contract_id = env.register(ExactChangeMall, ());
    let client = ExactChangeMallClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    let user = Address::random(&env);
    let merchant = Address::random(&env);

    client.init(&admin);
    client.register_merchant(&admin, &merchant);
    client.deposit(&admin, &user, &100);
    client.pay(&user, &merchant, &40);

    assert_eq!(client.get_balance(&user), 60);
    assert_eq!(client.get_balance(&merchant), 40);
}

#[test]
#[should_panic]
fn test_invalid_merchant() {
    let env = Env::default();
    let contract_id = env.register(ExactChangeMall, ());
    let client = ExactChangeMallClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    let user = Address::random(&env);
    let fake_merchant = Address::random(&env);

    client.init(&admin);
    client.deposit(&admin, &user, &50);
    client.pay(&user, &fake_merchant, &10);
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    let contract_id = env.register(ExactChangeMall, ());
    let client = ExactChangeMallClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    let user = Address::random(&env);

    client.init(&admin);
    client.deposit(&admin, &user, &70);

    assert_eq!(client.get_balance(&user), 70);
}

#[test]
#[should_panic]
fn test_unauthorized_deposit() {
    let env = Env::default();
    let contract_id = env.register(ExactChangeMall, ());
    let client = ExactChangeMallClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    let attacker = Address::random(&env);
    let user = Address::random(&env);

    client.init(&admin);
    client.deposit(&attacker, &user, &100);
}

#[test]
fn test_multiple_payments() {
    let env = Env::default();
    let contract_id = env.register(ExactChangeMall, ());
    let client = ExactChangeMallClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    let user = Address::random(&env);
    let merchant = Address::random(&env);

    client.init(&admin);
    client.register_merchant(&admin, &merchant);
    client.deposit(&admin, &user, &100);

    client.pay(&user, &merchant, &20);
    client.pay(&user, &merchant, &30);

    assert_eq!(client.get_balance(&user), 50);
    assert_eq!(client.get_balance(&merchant), 50);
}
#![cfg(test)]

use soroban_sdk::{
    testutils::Address as _,
    Address,
    Env,
};

use crate::{
    EduWalletContract,
    EduWalletContractClient,
};

#[test]
fn test_happy_path() {
    let env = Env::default();

    let contract_id = env.register(EduWalletContract, ());
    let client = EduWalletContractClient::new(
        &env,
        &contract_id,
    );

    let admin = Address::generate(&env);
    let student = Address::generate(&env);

    client.initialize(&admin);

    client.create_scholarship(
        &student,
        &100_i128,
    );

    client.request_withdrawal(&student);

    client.approve_request(&student);

    client.claim(&student);

    let status = client.get_status(&student);

    assert!(status.claimed);
}

#[test]
#[should_panic(expected = "withdrawal not requested")]
fn test_approve_without_request() {
    let env = Env::default();

    let contract_id = env.register(EduWalletContract, ());
    let client = EduWalletContractClient::new(
        &env,
        &contract_id,
    );

    let admin = Address::generate(&env);
    let student = Address::generate(&env);

    client.initialize(&admin);

    client.create_scholarship(
        &student,
        &100_i128,
    );

    client.approve_request(&student);
}

#[test]
fn test_state_verification() {
    let env = Env::default();

    let contract_id = env.register(EduWalletContract, ());
    let client = EduWalletContractClient::new(
        &env,
        &contract_id,
    );

    let admin = Address::generate(&env);
    let student = Address::generate(&env);

    client.initialize(&admin);

    client.create_scholarship(
        &student,
        &500_i128,
    );

    client.request_withdrawal(&student);

    let status = client.get_status(&student);

    assert_eq!(status.amount, 500);
    assert!(status.requested);
    assert!(!status.approved);
    assert!(!status.claimed);
}

#[test]
#[should_panic(expected = "scholarship already exists")]
fn test_duplicate_scholarship() {
    let env = Env::default();

    let contract_id = env.register(EduWalletContract, ());
    let client = EduWalletContractClient::new(
        &env,
        &contract_id,
    );

    let admin = Address::generate(&env);
    let student = Address::generate(&env);

    client.initialize(&admin);

    client.create_scholarship(
        &student,
        &100_i128,
    );

    client.create_scholarship(
        &student,
        &200_i128,
    );
}

#[test]
#[should_panic(expected = "already claimed")]
fn test_double_claim() {
    let env = Env::default();

    let contract_id = env.register(EduWalletContract, ());
    let client = EduWalletContractClient::new(
        &env,
        &contract_id,
    );

    let admin = Address::generate(&env);
    let student = Address::generate(&env);

    client.initialize(&admin);

    client.create_scholarship(
        &student,
        &100_i128,
    );

    client.request_withdrawal(&student);

    client.approve_request(&student);

    client.claim(&student);

    client.claim(&student);
}
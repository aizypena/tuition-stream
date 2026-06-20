#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env,
};

#[contract]
pub struct EduWalletContract;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Scholarship(Address),
}

#[derive(Clone)]
#[contracttype]
pub struct Scholarship {
    pub student: Address,
    pub amount: i128,
    pub requested: bool,
    pub approved: bool,
    pub claimed: bool,
}

#[contractimpl]
impl EduWalletContract {
    /// Initialize contract admin (foundation)
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().persistent().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        admin.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Admin, &admin);
    }

    /// Create a scholarship for a student
    pub fn create_scholarship(
        env: Env,
        student: Address,
        amount: i128,
    ) {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .unwrap();

        admin.require_auth();

        if env.storage()
            .persistent()
            .has(&DataKey::Scholarship(student.clone()))
        {
            panic!("scholarship already exists");
        }

        let scholarship = Scholarship {
            student: student.clone(),
            amount,
            requested: false,
            approved: false,
            claimed: false,
        };

        env.storage()
            .persistent()
            .set(
                &DataKey::Scholarship(student),
                &scholarship,
            );
    }

    /// Student requests withdrawal
    pub fn request_withdrawal(
        env: Env,
        student: Address,
    ) {
        student.require_auth();

        let mut scholarship: Scholarship = env
            .storage()
            .persistent()
            .get(&DataKey::Scholarship(student.clone()))
            .unwrap();

        if scholarship.claimed {
            panic!("already claimed");
        }

        scholarship.requested = true;

        env.storage()
            .persistent()
            .set(
                &DataKey::Scholarship(student),
                &scholarship,
            );
    }

    /// Foundation approves request
    pub fn approve_request(
        env: Env,
        student: Address,
    ) {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .unwrap();

        admin.require_auth();

        let mut scholarship: Scholarship = env
            .storage()
            .persistent()
            .get(&DataKey::Scholarship(student.clone()))
            .unwrap();

        if !scholarship.requested {
            panic!("withdrawal not requested");
        }

        scholarship.approved = true;

        env.storage()
            .persistent()
            .set(
                &DataKey::Scholarship(student),
                &scholarship,
            );
    }

    /// Student claims approved scholarship
    pub fn claim(
        env: Env,
        student: Address,
    ) {
        student.require_auth();

        let mut scholarship: Scholarship = env
            .storage()
            .persistent()
            .get(&DataKey::Scholarship(student.clone()))
            .unwrap();

        if !scholarship.approved {
            panic!("not approved");
        }

        if scholarship.claimed {
            panic!("already claimed");
        }

        scholarship.claimed = true;

        env.storage()
            .persistent()
            .set(
                &DataKey::Scholarship(student),
                &scholarship,
            );
    }

    /// View scholarship status
    pub fn get_status(
        env: Env,
        student: Address,
    ) -> Scholarship {
        env.storage()
            .persistent()
            .get(&DataKey::Scholarship(student))
            .unwrap()
    }
}
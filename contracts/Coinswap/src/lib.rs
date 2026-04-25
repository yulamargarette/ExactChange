#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, Map};

// Storage keys
const BALANCES: Symbol = Symbol::short("BAL");
const ADMIN: Symbol = Symbol::short("ADM");
const MERCHANTS: Symbol = Symbol::short("MRC");

#[contract]
pub struct ExactChangeMall;

#[contractimpl]
impl ExactChangeMall {

    // Initialize contract with admin (kiosk operator)
    pub fn init(env: Env, admin: Address) {
        admin.require_auth();

        let balances: Map<Address, i128> = Map::new(&env);
        let merchants: Map<Address, bool> = Map::new(&env);

        env.storage().instance().set(&BALANCES, &balances);
        env.storage().instance().set(&MERCHANTS, &merchants);
        env.storage().instance().set(&ADMIN, &admin);
    }

    // Register a merchant (cashier terminal)
    pub fn register_merchant(env: Env, admin: Address, merchant: Address) {
        admin.require_auth();

        let stored_admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        if admin != stored_admin {
            panic!("Not authorized");
        }

        let mut merchants: Map<Address, bool> =
            env.storage().instance().get(&MERCHANTS).unwrap();

        merchants.set(merchant, true);
        env.storage().instance().set(&MERCHANTS, &merchants);
    }

    // Deposit (kiosk converts cash → credits)
    pub fn deposit(env: Env, admin: Address, user: Address, amount: i128) {
        admin.require_auth();

        let stored_admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        if admin != stored_admin {
            panic!("Only kiosk can mint");
        }

        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&BALANCES).unwrap();

        let current = balances.get(user.clone()).unwrap_or(0);
        balances.set(user, current + amount);

        env.storage().instance().set(&BALANCES, &balances);
    }

    // Pay merchant only (supermarket checkout)
    pub fn pay(env: Env, user: Address, merchant: Address, amount: i128) {
        user.require_auth();

        let merchants: Map<Address, bool> =
            env.storage().instance().get(&MERCHANTS).unwrap();

        if !merchants.get(merchant.clone()).unwrap_or(false) {
            panic!("Invalid merchant");
        }

        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&BALANCES).unwrap();

        let user_balance = balances.get(user.clone()).unwrap_or(0);

        if user_balance < amount {
            panic!("Insufficient balance");
        }

        let merchant_balance = balances.get(merchant.clone()).unwrap_or(0);

        balances.set(user, user_balance - amount);
        balances.set(merchant, merchant_balance + amount);

        env.storage().instance().set(&BALANCES, &balances);
    }

    // Check balance
    pub fn get_balance(env: Env, user: Address) -> i128 {
        let balances: Map<Address, i128> =
            env.storage().instance().get(&BALANCES).unwrap();

        balances.get(user).unwrap_or(0)
    }
}
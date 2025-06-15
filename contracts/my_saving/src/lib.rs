use soroban_sdk::{contract, contractimpl, contracttype, BytesN, Env, Vec};
// must implement an owner validation service for pausing and unpausing
//  pause only done on Widthdrawal
//ByteN is for fixed length array , whose the oppose of vec
#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct DepositeEvent {
    pub user_pubkey: BytesN<32>,
    pub amount: u64,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct WidthdrawalEvent {
    pub user_pubkey: BytesN<32>,
    pub amount: u64,
    pub is_paused: bool,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Debug)]
pub enum DataKeys {
    Deposite,
    Widthdrawal,
    User,
}

#[contracttype]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct UserAcc {
    pubkey: BytesN<32>,
    balance: u64,
}

#[contractimpl]
impl Contract {
    pub fn deposite(env: Env, user_pubkey: BytesN<32>, amount: u64) {
        let _instance = env.storage().instance();
        let mut dp_list: Vec<DepositeEvent> =
            _instance.get(&DataKeys::Deposite).unwrap_or(Vec::new(&env));
        let new_dp = DepositeEvent {
            user_pubkey: user_pubkey.clone(),
            amount: amount.clone(),
            timestamp: env.ledger().timestamp(),
        };

        dp_list.push_front(new_dp);
        _instance.set(&DataKeys::Deposite, &dp_list);
        update_balance(env.clone(), user_pubkey.clone(), amount.clone());
        //  Ok(())
    }

    pub fn get_deposites(env: Env, _user_pubkey: BytesN<32>) -> Vec<DepositeEvent> {
        let _instance = env.storage().instance();
        let dps: Vec<DepositeEvent> = _instance.get(&DataKeys::Deposite).unwrap_or(Vec::new(&env));
        let mut result = Vec::new(&env);
        for dp in dps.iter() {
            if dp.user_pubkey == _user_pubkey {
                result.push_back(dp);
            }
        }
        result
    }
    pub fn get_balance(env: Env, _user_pubkey: BytesN<32>) -> u64 {
        match get_user(env.clone(), _user_pubkey.clone()) {
            Some(user) => user.balance,
            None => 0,
        }
    }
    pub fn get_all_deposites(env: Env) -> Vec<DepositeEvent> {
        let _instance = env.storage().instance();
        _instance
            .get::<_, Vec<DepositeEvent>>(&DataKeys::Deposite)
            .unwrap_or(Vec::new(&env))
    }

    pub fn widthdrawal(env: Env, _user_pubkey: BytesN<32>, _amount: u64) {
        let _instance = env.storage().instance();
    }
}
fn get_user(env: Env, _user_pubkey: BytesN<32>) -> Option<UserAcc> {
    let _instance = env.storage().instance();
    let users: Vec<UserAcc> = _instance.get(&DataKeys::User).unwrap_or(Vec::new(&env));
    for user in users.iter() {
        if user.pubkey == _user_pubkey {
            Some(user);
        }
    }
    None
}
fn update_balance(env: Env, _user_pubkey: BytesN<32>, new_balance: u64) {
    let _instance = env.storage().instance();
    let mut users: Vec<UserAcc> = _instance.get(&DataKeys::User).unwrap_or(Vec::new(&env));
    let mut found = false;

    for i in 0..users.len() {
        if let Some(user) = users.get(i) {
            if user.pubkey == _user_pubkey {
                let updated_user = UserAcc {
                    pubkey: _user_pubkey.clone(),
                    balance: new_balance.clone(),
                };
                users.set(i, updated_user);
                found = true;
                break;
            }
        }
    }

    if !found {
        users.push_front(UserAcc {
            pubkey: _user_pubkey.clone(),
            balance: new_balance.clone(),
        });
    }
    _instance.set(&DataKeys::User, &users);
    drop(_instance);
}
mod test;

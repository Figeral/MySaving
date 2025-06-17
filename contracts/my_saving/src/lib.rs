use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};
// must implement an owner validation service for pausing and unpausing
//  pause only done on Widthdrawal
//ByteN is for fixed length array , whose the oppose of vec
#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone, Debug)]
pub struct DepositeEvent {
    pub nonce: u64,
    pub user_addr: Address,
    pub amount: u64,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Debug, Clone)]
pub struct WidthdrawalEvent {
    pub nonce: u64,
    pub user_addr: Address,
    pub amount: u64,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Debug)]
pub enum DataKeys {
    Deposite,
    Widthdrawal,
    User,
    Owner,
    PausedWidthdrawal,
}

#[contracttype]
#[derive(Debug)]
pub enum Actions {
    Pause,
    Unpause,
}

#[contracttype]
#[derive(Debug, Clone)]
struct UserAcc {
    addr: Address,
    balance: i64,
}

#[contractimpl]
impl Contract {
    pub fn initialize(env: Env, owner: Address) {
        let _instance = env.storage().instance();
        if _instance.has(&DataKeys::Owner) {
            panic!("An Owner is already set who's : {:?}", owner);
        }
        _instance.set(&DataKeys::PausedWidthdrawal, &false);
        _instance.set(&DataKeys::Owner, &owner);
    }

    pub fn deposite(env: Env, user_addr: Address, amount: u64) {
        let _instance = env.storage().instance();
        let mut dp_list: Vec<DepositeEvent> =
            _instance.get(&DataKeys::Deposite).unwrap_or(Vec::new(&env));
        let nonce = dp_list.first().map(|dp| dp.nonce + 1).unwrap_or(0);
        let new_dp = DepositeEvent {
            nonce: nonce,
            user_addr: user_addr.clone(),
            amount: amount.clone(),
            timestamp: env.ledger().timestamp(),
        };
        dp_list.push_back(new_dp);
        _instance.set(&DataKeys::Deposite, &dp_list);
        update_balance(
            env.clone(),
            user_addr.clone(),
            i64::try_from(amount.clone()).expect("failed to convert value to signed"),
        );
        //  Ok(())
    }

    pub fn get_deposites(env: Env, user_addr: Address) -> Vec<DepositeEvent> {
        let _instance = env.storage().instance();
        let dps: Vec<DepositeEvent> = _instance.get(&DataKeys::Deposite).unwrap_or(Vec::new(&env));
        let mut result = Vec::new(&env);
        for dp in dps.iter() {
            if dp.user_addr == user_addr {
                result.push_back(dp);
            }
        }
        result
    }
    pub fn get_balance(env: Env, user_addr: Address) -> i64 {
        match get_user(env.clone(), user_addr.clone()) {
            Some(user) => user.balance,
            None => 0,
        }
    }

    pub fn get_all_deposites(env: Env, owner_addr: Address) -> Vec<DepositeEvent> {
        let _instance = env.storage().instance();
        let stored_owner = _instance.get::<_, Address>(&DataKeys::Owner);
        if stored_owner != Some(owner_addr.clone()) {
            panic!("Only the contract owner can view all the deposites ")
        }
        _instance
            .get::<_, Vec<DepositeEvent>>(&DataKeys::Deposite)
            .unwrap_or(Vec::new(&env))
    }

    pub fn widthdrawal(env: Env, user_addr: Address, amount: u64) {
        let _instance = env.storage().instance();
        let is_paused = _instance
            .get::<_, bool>(&DataKeys::PausedWidthdrawal)
            .unwrap_or(false);

        if is_paused {
            panic!("Withdrawals are currently paused.");
        }
        let mut wd_list: Vec<WidthdrawalEvent> = _instance
            .get(&DataKeys::Widthdrawal)
            .unwrap_or(Vec::new(&env));
        let nonce = wd_list.first().map(|wd| wd.nonce + 1).unwrap_or(0);
        let new_dp = WidthdrawalEvent {
            nonce: nonce,
            user_addr: user_addr.clone(),
            amount: amount.clone(),
            timestamp: env.ledger().timestamp(),
        };

        wd_list.push_front(new_dp);
        _instance.set(&DataKeys::Deposite, &wd_list);
        update_balance(
            env.clone(),
            user_addr.clone(),
            -(i64::try_from(amount.clone()).expect("failed to convert value to signed")),
        );
    }
    pub fn get_widthdrawal(env: Env, user_addr: Address) -> Vec<WidthdrawalEvent> {
        let _instance = env.storage().instance();
        let dps: Vec<WidthdrawalEvent> = _instance
            .get(&DataKeys::Widthdrawal)
            .unwrap_or(Vec::new(&env));
        let mut result = Vec::new(&env);
        for dp in dps.iter() {
            if dp.user_addr == user_addr {
                result.push_back(dp);
            }
        }
        result
    }
    pub fn get_all_widthdrawal(env: Env, owner_addr: Address) -> Vec<WidthdrawalEvent> {
        let _instance = env.storage().instance();
        let stored_owner = get_owner(&env);
        if stored_owner != Some(owner_addr.clone()) {
            panic!("Only the contract owner can view all the widthdrawal ")
        }
        _instance
            .get::<_, Vec<WidthdrawalEvent>>(&DataKeys::Deposite)
            .unwrap_or(Vec::new(&env))
    }
    pub fn pause_widthdrawal(env: Env, owner_addr: Address, action: Actions) -> bool {
        let _instance = env.storage().instance();
        if !is_owner(&env, &owner_addr) {
            panic!("Only owner can pause/unpause withdrawals");
        }
        match action {
            Actions::Pause => _instance.set(&DataKeys::PausedWidthdrawal, &true),
            Actions::Unpause => _instance.set(&DataKeys::PausedWidthdrawal, &false),
        };
        Self::is_pause_widthdrawal(env.clone())
    }
    pub fn is_pause_widthdrawal(env: Env) -> bool {
        let _instance = env.storage().instance();
        _instance
            .get::<_, bool>(&DataKeys::PausedWidthdrawal)
            .unwrap_or(false)
    }
}
fn get_user(env: Env, user_pubkey: Address) -> Option<UserAcc> {
    let _instance = env.storage().instance();
    let users: Vec<UserAcc> = _instance.get(&DataKeys::User).unwrap_or(Vec::new(&env));
    for user in users.iter() {
        if user.addr == user_pubkey {
            return Some(user);
        }
    }
    None
}
fn get_owner(env: &Env) -> Option<Address> {
    let _instance = env.storage().instance();
    _instance.get::<_, Address>(&DataKeys::Owner)
}
fn is_owner(env: &Env, owner: &Address) -> bool {
    match get_owner(env) {
        Some(o) => o == owner.to_owned(),
        None => false,
    }
}
fn update_balance(env: Env, user_addr: Address, new_balance: i64) {
    let _instance = env.storage().instance();
    let mut users: Vec<UserAcc> = _instance.get(&DataKeys::User).unwrap_or(Vec::new(&env));
    let mut found = false;
    for i in 0..users.len() {
        if let Some(user) = users.get(i) {
            if user.addr == user_addr {
                let updated_user = UserAcc {
                    addr: user_addr.clone(),
                    balance: user.balance + new_balance.clone(),
                };
                users.set(i, updated_user);
                found = true;
                break;
            }
        }
    }

    if !found {
        users.push_front(UserAcc {
            addr: user_addr.clone(),
            balance: new_balance.clone(),
        });
    }
    _instance.set(&DataKeys::User, &users);
    drop(_instance);
}
mod test;

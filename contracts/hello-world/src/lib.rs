#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, log, String};

#[contracttype]
#[derive(Clone)]
pub struct WalletInfo {
    pub owner: Address,
    pub label: String,
    pub connected_at: u64,
}

#[contracttype]
pub enum DataKey {
    Wallets(Address),
}

#[contract]
pub struct MultiWalletContract;

#[contractimpl]
impl MultiWalletContract {
    pub fn connect_wallet(env: Env, wallet: Address, label: String) {
        wallet.require_auth();
        let mut wallets: Vec<WalletInfo> = env
            .storage()
            .instance()
            .get(&DataKey::Wallets(wallet.clone()))
            .unwrap_or(Vec::new(&env));

        let info = WalletInfo {
            owner: wallet.clone(),
            label,
            connected_at: env.ledger().timestamp(),
        };

        wallets.push_back(info);
        env.storage().instance().set(&DataKey::Wallets(wallet), &wallets);
        log!(&env, "Wallet connected.");
    }

    pub fn list_wallets(env: Env, user: Address) -> Vec<WalletInfo> {
        env.storage()
            .instance()
            .get(&DataKey::Wallets(user))
            .unwrap_or(Vec::new(&env))
    }

    pub fn disconnect_all_wallets(env: Env, user: Address) {
        user.require_auth();
        env.storage().instance().remove(&DataKey::Wallets(user.clone()));
        log!(&env, "Disconnected all wallets for user {:?}", user);
    }
}

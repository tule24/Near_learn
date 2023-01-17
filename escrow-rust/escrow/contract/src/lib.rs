use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::{log, near_bindgen, AccountId, BorshStorageKey, require, env, Promise, PromiseError, Gas};

pub mod external;
pub use crate::external::*;

const GAS_FEE: u64 = 30_000_000_000_000; // 30 TGAS

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct EscrowContract {
    account_receivers: LookupMap<AccountId, AccountId>,             // Mapping buyer_account_id => seller_account_id
    account_value_locked: LookupMap<AccountId, U128>,               // Mapping buyer_account_id => near_attached_amount
    account_assets: LookupMap<AccountId, U128>,                     // Mapping buyer_account_id => buyer_assets
    account_time_created: UnorderedMap<AccountId, u64>,             // Mapping buyer_account_id => block_timestamp
    account_asset_contract_id: LookupMap<AccountId, AccountId>      // Mappin buyer_account_id => asset_contract_id
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum EscrowKeys {
    Receiver,
    ValueLocked,
    Assets,
    TimeCreated,
    AssetsContractId
}

impl Default for EscrowContract {
    fn default() -> Self {
        Self { 
            account_receivers: LookupMap::new(EscrowKeys::Receiver), 
            account_value_locked: LookupMap::new(EscrowKeys::ValueLocked), 
            account_assets: LookupMap::new(EscrowKeys::Assets), 
            account_time_created: UnorderedMap::new(EscrowKeys::TimeCreated), 
            account_asset_contract_id: LookupMap::new(EscrowKeys::AssetsContractId) 
        }
    }
}

#[near_bindgen]
impl EscrowContract {
    #[private]
    pub fn send_near(receiving_account_id: AccountId, amount: U128) {
        require!(u128::from(amount) > 0, "The amount should be a positive number");
        require!(receiving_account_id != env::current_account_id(), "Can't transfer to the contract itself");
        require!(u128::from(amount) < env::account_balance(), "Not enough balance to cover transfer of yoctoNEAR");

        Promise::new(receiving_account_id).transfer(u128::from(amount));
    }

    #[private]
    pub fn completed_near_transaction(&mut self, seller_account: AccountId, amount: U128, buyer_account: AccountId) {
        Self::send_near(seller_account, amount);
        self.account_receivers.remove(&buyer_account);
        self.account_value_locked.remove(&buyer_account);
        self.account_assets.remove(&buyer_account);
        self.account_asset_contract_id.remove(&buyer_account);
        self.account_time_created.remove(&buyer_account);
    }

    #[private]
    pub fn cross_contract_transfer_asset(&mut self, asset_contract_id: AccountId, quantity: U128, from_account_id: AccountId, to_account_id: AccountId) -> Promise{
        assets::ext(asset_contract_id)
                .with_static_gas(Gas(GAS_FEE))
                .transfer_asset(quantity, from_account_id, to_account_id)
        .then(
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(GAS_FEE))
            .transfer_asset_callback()
        )
    }

    #[private]
    pub fn transfer_asset_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>) -> bool {
        if call_result.is_err() {
            env::log_str("transfer asset failed...");
            return false
        } 
        env::log_str("transfer asset success");
        return true
    }

    #[payable]
    pub fn purchase_in_escrow(&mut self, seller_account_id: AccountId, asset_contract_id: AccountId) -> Promise{
        let attached_amount = env::attached_deposit();
        let amount = attached_amount - u128::from(GAS_FEE);
        let buyer_account_id = env::predecessor_account_id();
        require!(amount > 0, "Must attach a positive amount");
        require!(!self.account_value_locked.contains_key(&buyer_account_id), "Can't escrow purchase twice before completing one first: feature not implemented");
        require!(seller_account_id != buyer_account_id, "Can't escrow to the same account");
        require!(buyer_account_id != env::current_account_id(), "Can't escrow from the contract itself");

        self.account_receivers.insert(&buyer_account_id, &seller_account_id);
        self.account_value_locked.insert(&buyer_account_id, &U128(attached_amount));
        self.account_asset_contract_id.insert(&buyer_account_id, &asset_contract_id);
        self.account_time_created.insert(&buyer_account_id, &env::block_timestamp());
        self.account_assets.insert(&buyer_account_id, &U128(0));

        assets::ext(asset_contract_id)
                .with_static_gas(Gas(GAS_FEE))
                .purcharse_asset(seller_account_id, buyer_account_id.clone(), U128(amount))
        .then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(GAS_FEE))
                .purchase_escrow_callback(buyer_account_id.clone())   
        )
    }

    #[private]
    pub fn purchase_escrow_callback(&mut self, buyer_account_id: AccountId, #[callback_result] call_result: Result<U128, PromiseError>) -> bool{
        if call_result.is_err() {
            let amount = self.account_value_locked.get(&buyer_account_id).unwrap_or(U128(0));
            env::log_str("Escrow purchase failed, returning yoctoNEAR to buyer");
            self.completed_near_transaction(buyer_account_id.clone(), amount, buyer_account_id.clone());
            return false;
        } else {
            env::log_str("Purcharse asset success...");
            let quantity = call_result.unwrap();
            self.account_assets.insert(&buyer_account_id, &quantity);
            return true;
        }
    }

    pub fn escrow_timeout_scan(&mut self) {
        let caller_id = env::predecessor_account_id();
        let timeout: u64 = if caller_id == "test.near".parse().unwrap() {0} else { 86_400_000_000_000 }; // 24 hours in nanoseconds. Testing workaround until fast-forward is implemented in workspaces-js
        let all_account_time = self.account_time_created.to_vec();
        for (buyer_account_id, time_created) in all_account_time{
            if time_created + timeout < env::block_timestamp() {
                let receiver_id = self.account_receivers.get(&buyer_account_id).unwrap();
                let amount = self.account_value_locked.get(&buyer_account_id).unwrap_or(U128(0));
                self.completed_near_transaction(receiver_id, amount, buyer_account_id);
            }
        }
    }

    pub fn approve_purchase(&mut self) {
        let buyer_account_id = env::predecessor_account_id();
        require!(self.account_value_locked.contains_key(&buyer_account_id), "Can't approve escrow purchase before escrowing");
        let seller_account_id = self.account_receivers.get(&buyer_account_id);
        let amount = self.account_value_locked.get(&buyer_account_id);
        if seller_account_id.is_some() && amount.is_some() {
            self.completed_near_transaction(seller_account_id.unwrap(), amount.unwrap(), buyer_account_id);
        } else {
            log!("Seller or amount is not found")
        }
    }

    pub fn cancel_purchase(&mut self) {
        let buyer_account_id = env::predecessor_account_id();
        let amount = self.account_value_locked.get(&buyer_account_id).unwrap_or(U128(0));
        require!(amount > U128(0), "No escrow purchase found for this buyer");
        let seller_account_id = self.account_receivers.get(&buyer_account_id);
        let asset_contract_id = self.account_asset_contract_id.get(&buyer_account_id);
        let quantity = self.account_assets.get(&buyer_account_id);

        if seller_account_id.is_some() && asset_contract_id.is_some() && quantity.is_some() {
            let seller_account_id = seller_account_id.unwrap();
            let quantity = quantity.unwrap();
            let asset_contract_id = asset_contract_id.unwrap();

            self.completed_near_transaction(buyer_account_id.clone(), amount, buyer_account_id.clone()); // return funds to buyer
            self.cross_contract_transfer_asset(asset_contract_id, quantity, buyer_account_id.clone(), seller_account_id.clone());
        }
    }

    pub fn view_pending_escrow(&self, account_id: AccountId) -> Option<(AccountId, U128, u64)>{
        let receiver_id = self.account_receivers.get(&account_id);
        let amount = self.account_value_locked.get(&account_id);
        let time_created = self.account_time_created.get(&account_id);
        if receiver_id.is_some() && amount.is_some() && time_created.is_some() {
            return Some((receiver_id.unwrap(), amount.unwrap(), time_created.unwrap()))
        }
        return None
    }
}

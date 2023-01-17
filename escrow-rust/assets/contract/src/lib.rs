use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{env, require, log, near_bindgen, AccountId};
use near_sdk::json_types::U128;
use near_sdk::collections::LookupMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AssetContract {
    asset_price: U128,
    escrow_contract_id: AccountId,
    total_supply: U128,
    account_assets: LookupMap<AccountId, U128>
}

impl Default for AssetContract {
    fn default() -> Self {
        Self { asset_price: U128(0), escrow_contract_id: env::current_account_id(), total_supply: U128(0), account_assets: LookupMap::new(b'a') }
    }
}

#[near_bindgen]
impl AssetContract {
    #[init]
    pub fn init(asset_price: U128, escrow_contract_id: AccountId, total_supply: U128, owner_id: AccountId) -> Self {
        log!("Asset contract initialization!");
        let mut account_assets = LookupMap::new(b'a');
        account_assets.insert(&owner_id, &total_supply);
        Self { asset_price, escrow_contract_id, total_supply, account_assets }
    }

    pub fn get_total_supply(&self) -> U128 {
        self.total_supply
    }

    pub fn get_account_assets(&self, account_id: AccountId) -> U128 {
        self.account_assets.get(&account_id).unwrap_or(U128(0))
    }

    pub fn purcharse_asset(&mut self, seller_account_id: AccountId, buyer_account_id: AccountId, attached_near: U128) -> U128{
        require!(env::predecessor_account_id() == self.escrow_contract_id, "Only escrow contract can call this method");
        require!(self.account_assets.contains_key(&seller_account_id), "Seller does not own any assets");
        require!(u128::from(self.asset_price) <= u128::from(attached_near), "Attached Near is not enough to buy the asset");

        let quantity = u128::from(attached_near) / (u128::from(self.asset_price));
        let seller_assets = u128::from(self.account_assets.get(&seller_account_id).unwrap_or(U128(0)));
        require!(seller_assets > quantity, "Seller does not own enough assets of required assets");

        let seller_new_assets = seller_assets - quantity;
        self.account_assets.insert(&seller_account_id, &U128(seller_new_assets));

        let receiving_account_new_assets = u128::from(self.account_assets.get(&buyer_account_id).unwrap_or(U128(0))) + quantity;
        self.account_assets.insert(&buyer_account_id, &U128(receiving_account_new_assets));

        return U128(quantity)
    }

    pub fn transfer_asset(&mut self, quantity: U128, from_account_id: AccountId, to_account_id: AccountId) {
        require!(env::predecessor_account_id() == self.escrow_contract_id, "Only escrow contract can call this method");
        require!(self.account_assets.contains_key(&from_account_id), "Sender does not own any assets");
        
        let sender_assets = u128::from(self.account_assets.get(&from_account_id).unwrap_or(U128(0)));
        require!(sender_assets >= u128::from(quantity), "Sender does not own enough assets of required assets");

        let sender_new_assets = sender_assets - u128::from(quantity);
        self.account_assets.insert(&from_account_id, &U128(sender_new_assets));

        let receiving_account_new_assets = u128::from(self.account_assets.get(&to_account_id).unwrap_or(U128(0))) + u128::from(quantity);
        self.account_assets.insert(&to_account_id, &U128(receiving_account_new_assets));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OWNER_A: &str = "owner_a";
    const OWNER_B: &str = "owner_b";

    #[test]
    fn test_init() {
        let contract = AssetContract::init(U128(10), env::predecessor_account_id(), U128(100_000), OWNER_A.parse().unwrap());
        assert_eq!(contract.escrow_contract_id, env::predecessor_account_id());
        assert_eq!(contract.get_total_supply(), U128(100_000));
        assert_eq!(contract.get_account_assets(OWNER_A.parse().unwrap()), U128(100_000));
    }

    #[test]
    fn test_purcharse_asset() {
        let mut contract = AssetContract::init(U128(10), env::predecessor_account_id(), U128(100), OWNER_A.parse().unwrap());
        contract.purcharse_asset(OWNER_A.parse().unwrap(), OWNER_B.parse().unwrap(), U128(100));
        assert_eq!(contract.get_account_assets(OWNER_A.parse().unwrap()), U128(90));
        assert_eq!(contract.get_account_assets(OWNER_B.parse().unwrap()), U128(10));
    }

    #[test]
    fn test_transfer_asset() {
        let mut contract = AssetContract::init(U128(10), env::predecessor_account_id(), U128(100), OWNER_A.parse().unwrap());
        contract.transfer_asset(U128(50), OWNER_A.parse().unwrap(), OWNER_B.parse().unwrap());
        assert_eq!(contract.get_account_assets(OWNER_A.parse().unwrap()), U128(50));
        assert_eq!(contract.get_account_assets(OWNER_B.parse().unwrap()), U128(50));
    }
}

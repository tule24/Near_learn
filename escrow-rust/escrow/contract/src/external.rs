use near_sdk::{ext_contract, json_types::U128, AccountId};

#[ext_contract(assets)]
trait Assets {
    fn get_total_supply(&self) -> U128;
    fn get_account_assets(&self, account_id: AccountId) -> U128;
    fn purcharse_asset(&mut self, seller_account_id: AccountId, buyer_account_id: AccountId, attached_near: U128);
    fn transfer_asset(&mut self, quantity: U128, from_account_id: AccountId, to_account_id: AccountId);
}
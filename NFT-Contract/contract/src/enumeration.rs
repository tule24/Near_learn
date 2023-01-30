use crate::*;
use crate::nft_core::NonFungibleTokenCore;

#[near_bindgen]
impl Contract {
    //Query for the total supply of NFTs on the contract
    pub fn nft_total_supply(&self) -> U128 {
        U128(self.token_metadata_by_id.len() as u128)
    }

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.token_metadata_by_id
            .keys()
            .skip(from as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }

    //get the total supply of NFTs for a given owner
    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        let tokens = self.tokens_by_owner.get(&account_id);
        if let Some(tokens) = tokens {
            U128(tokens.len() as u128)
        } else {
            U128(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn nft_tokens_for_owner(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_by_owner.get(&account_id);
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector.
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let from = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        tokens
            .iter()
            .skip(from as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }
}
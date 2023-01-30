use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise ,
};
use std::collections::HashMap;

mod approval;
mod enumeration;
mod metadata;
mod mint;
mod nft_core;
mod royalty;
mod internal;
mod events;

pub use crate::approval::*;
pub use crate::enumeration::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::royalty::*;
use crate::internal::*;
pub use crate::events::*;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,                       // contract owner
    pub metadata: LazyOption<NFTContractMetadata>, // Metadata of NFT Contract

    pub tokens_by_owner: LookupMap<AccountId, UnorderedSet<TokenId>>, // Mapping owner => ds tokenId
    pub token_by_id: LookupMap<TokenId, Token>,                       // Mapping tokenId => token
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>, // Mapping tokenId => token metadata
}

// Helper structure for keys of the persistent collections
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokenById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        // Invoke when you first deploy the contract
        // create a variable of type Self with all the fields initialized
        Self {
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            tokens_by_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            token_by_id: LookupMap::new(StorageKey::TokenById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
        }
    }

    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in
        Self::new(
            owner_id, 
            NFTContractMetadata {
                spec: "nft-1.0.0".to_owned(),
                name: "NFT Tutorial Contract".to_owned(),
                symbol: "GOTEAM".to_owned(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None
            }
        )
    }
}

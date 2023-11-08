use near_contract_standards::impl_non_fungible_token_core;
use near_contract_standards::non_fungible_token::{
    metadata::NFTContractMetadata, NonFungibleToken, Token, TokenId,
};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, PromiseOrValue};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    token: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    ContractMetadata,
    OwnerById,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();

        let token = NonFungibleToken::new(
            StorageKey::OwnerById,
            owner_id.clone(),
            Some(StorageKey::TokenMetadata),
            Some(StorageKey::Enumeration),
            Some(StorageKey::Approval),
        );

        let this = Self {
            owner_id: owner_id.clone(),
            token,
            metadata: LazyOption::new(StorageKey::ContractMetadata, Some(&metadata)),
        };

        this
    }
}

impl_non_fungible_token_core!(Contract, token);

use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};

use near_sdk::{
    env, near_bindgen, AccountId,  CryptoHash, PanicOnDefault,  BorshStorageKey
};

mod tournament;
use crate::tournament::*;

mod macros;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {    
    owner_id: AccountId,
    tournament: TournamentContract,
    metadata: LazyOption<TournamentContractMetadata>, 
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    PlayersPerTournament,
    WinnersPercentPerTournament,
    PlayersPerTournamentInner { tournament_id_hash: CryptoHash },
    TournamentsById,
    TournamentMetadataById,
    TournamentContractMetadata,   
    PrizesPerTournamentInner { tournament_id_hash: CryptoHash },   
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            TournamentContractMetadata {                
                name: "Tournament Test Contract".to_string(),                
                icon: None,                
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: TournamentContractMetadata) -> Self {
        let metadata = LazyOption::new(
            StorageKey::TournamentContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
        );
        
        let tournament = TournamentContract::new(            
            StorageKey::PlayersPerTournament,            
            StorageKey::WinnersPercentPerTournament,           
            StorageKey::TournamentsById,            
            StorageKey::TournamentMetadataById,
        );
        
        Self {
            owner_id: owner_id.clone(),
            tournament,
            metadata,
        }
    }
    
    pub fn create(
        &mut self,
        tournament_id: TournamentId,
        name: String,
        icon: Option<String>,
        players_number: u8,
        in_price: U128,        
        tournament_owner_id: AccountId,
        percents_map: HashMap<u8,u8>,
    ) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can create tournaments");
        self.tournament.tournament_create(tournament_id, name, icon,  players_number,    in_price, tournament_owner_id, percents_map);
        
    }
}

pub trait ContractMetadata {
    //view call for returning the contract metadata
    fn contract_metadata(&self) -> TournamentContractMetadata;
}

#[near_bindgen]
impl ContractMetadata for Contract {
    fn contract_metadata(&self) -> TournamentContractMetadata {
        self.metadata.get().unwrap()
    }
}



impl_tournament_contract_core!(Contract, tournament);
impl_tournament_contract_enumeration!(Contract, tournament);

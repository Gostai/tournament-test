use crate::*;
use near_sdk::json_types::{U64, U128};

pub type TournamentId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentContractMetadata {
    pub name: String,                
    pub icon: Option<String>,               
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentMetadata {
    pub name: String,                
    pub icon: Option<String>,
    pub players_number: u8,
    pub in_price: u128,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tournament {
    pub owner_id: AccountId,
    pub active: bool,
    pub balance: u128,
}

//The Json tournament is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonTournament {
    //tournament ID
    pub tournament_id: TournamentId,
    //owner of the tournament
    pub owner_id: AccountId,
    //tournament metadata
    pub metadata: TournamentMetadata,
    
    pub first_place_prize: U64,
    
    pub second_place_prize: U64,
    
    pub third_place_prize: U64,
    
    //is the tournament active
    pub active: bool,
    
    //total prize fond for the tournament
    pub prize_fond: U128,    
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

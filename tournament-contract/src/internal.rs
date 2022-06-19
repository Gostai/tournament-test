use crate::*;
use near_sdk::{CryptoHash};
//use std::mem::size_of;

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_tournament_id(tournament_id: &TournamentId, shift: &String) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the tournament ID with shift and return it
    hash.copy_from_slice(&env::sha256((tournament_id.to_owned()+shift).as_bytes()));
    hash
}

impl Contract {
    //add prize values in percents to the tournament
    pub(crate) fn internal_add_prizes_to_tournament(
        &mut self,
        tournament_id: &TournamentId,
        percents_map: &HashMap<u8,u8>
        
    ) {
        //get the map of prizes for the given tournament
        let mut prizes_map = self.winners_percents_per_tournament.get(tournament_id).unwrap_or_else(|| {
            //if the tournament doesn't have any players, we create a new map 
            LookupMap::new(
                StorageKey::PrizesPerTournamentInner {
                    //we get a new unique prefix for the collection
                    tournament_id_hash: hash_tournament_id(&tournament_id, &"m".to_string()),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        for (key, value) in percents_map {
            prizes_map.insert(&key, &value);
        }
        

        //we insert that map for the given tournament ID. 
        self.winners_percents_per_tournament.insert(tournament_id, &prizes_map);
    }
    
    
    //add a player to the set of players the tournament has
    pub(crate) fn internal_add_player_to_tournament(
        &mut self,
        tournament_id: &TournamentId,
        player: &AccountId,        
    ) ->bool {
    
        //get the set of players for the given tournament
        let mut players_set = self.players_per_tournament.get(tournament_id).unwrap_or_else(|| {
            //if the tournament doesn't have any players, we create a new unordered set
            UnorderedSet::new(
                StorageKey::PlayersPerTournamentInner {
                    //we get a new unique prefix for the collection
                    tournament_id_hash: hash_tournament_id(&tournament_id, &"s".to_string()),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //we insert the player ID into the set
        let new_one = players_set.insert(player);

        //we insert that set for the given tournament ID. 
        self.players_per_tournament.insert(&tournament_id, &players_set);
        new_one
    }
    
    //get number of players already in the tournament
    pub(crate) fn internal_get_players_number_in_tournament(
        &self,
        tournament_id: &TournamentId,     
    ) -> u8 {
    
        //get the set of tokens for the given account
        if let Some (players_set) = self.players_per_tournament.get(tournament_id) {
            players_set.len() as u8
        } else { 0 }   
    }
} 


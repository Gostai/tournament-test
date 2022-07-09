use crate::*;

pub trait TournamentContractEnumeration{
    //Query for  tournaments on the contract regardless of the ID using pagination
    fn display_tournaments(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonTournament>;
}

impl TournamentContractEnumeration for TournamentContract {    
    //Query for  tournaments on the contract regardless of the ID using pagination
    fn display_tournaments(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonTournament> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each tournament using an iterator
        self.tournament_metadata_by_id.keys()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|tournament_id| self.display_tournament(tournament_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }    
}

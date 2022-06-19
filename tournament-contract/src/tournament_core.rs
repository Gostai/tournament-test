use crate::*;


pub trait TournamentContractCore {
        
    //get the information for a specific tournament ID
    fn display_tournament(&self, tournament_id: TournamentId) -> Option<JsonTournament>;
    
    //add player to the tournament with NEAR depositing
    fn participate_tournament(&mut self, tournament_id: TournamentId);
    
    //get free playses in the tournament
    fn display_freeplaces_in_tournament(&self, tournament_id: TournamentId) -> Option<U64>;
    
    //refunds the prizes for the winners 
    fn reward_prizes(&mut self, tournament_id: TournamentId, winners_map: HashMap<u8,AccountId>);
}


#[near_bindgen]
impl TournamentContractCore for Contract {

    //get the information for a specific tournament ID
    fn display_tournament(&self, tournament_id: TournamentId) -> Option<JsonTournament> {
        //if there is some token ID in the tokens_by_id collection
        if let Some(tournament) = self.tournaments_by_id.get(&tournament_id) {
            //we'll get the metadata for that token
            let metadata = self.tournament_metadata_by_id.get(&tournament_id).unwrap();
            
            let prizes = self.winners_percents_per_tournament.get(&tournament_id).unwrap();
            //we return the JsonToken (wrapped by Some since we return an option)
            Some(JsonTournament {
                tournament_id,
                owner_id: tournament.owner_id,
                metadata,
                first_place_prize: (prizes.get(&1).unwrap() as u64).into(),
                second_place_prize: (prizes.get(&2).unwrap() as u64).into(),
                third_place_prize: (prizes.get(&3).unwrap() as u64).into(),
                active: tournament.active,
                prize_fond: tournament.balance.into(),
            })
        } else { //if there wasn't a token ID in the tokens_by_id collection, we return None
            None
        }        
        
    }
    
    //add player to the tournament with NEAR depositing
    #[payable]
    fn participate_tournament(&mut self, tournament_id: TournamentId)  {
    
        let account_id: &AccountId = &env::predecessor_account_id();
        let attached_deposit: Balance = env::attached_deposit();  
        
        if let Some(mut tournament) = self.tournaments_by_id.get(&tournament_id) {
            //check activeness of the tournament
            assert!(tournament.active, "Tournament is inactive");
            
            //we'll get the metadata for that tournament
            let metadata = self.tournament_metadata_by_id.get(&tournament_id).unwrap();
            
            //Check there are some free playses for the players in the tournament
            assert!(
                metadata.players_number-self.internal_get_players_number_in_tournament(&tournament_id)>0,
                "Tournament is already full of players",
            );
            
            //check the is enouph deposit attached to players account
            assert!(attached_deposit >= metadata.in_price, "Deposit is too small. Attached: {}, Required: {}", attached_deposit, metadata.in_price);
            
            //check for double participation
            assert!(self.internal_add_player_to_tournament(&tournament_id,&account_id), "Already in the tournament");
            
            //save the prize fond balanse of the tournament 
            tournament.balance+=metadata.in_price;
            self.tournaments_by_id.insert(&tournament_id, &tournament);
            
            //get the refund amount from the attached deposit - required cost
            let refund = attached_deposit - metadata.in_price;
            
            //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
            if refund > 1 {
                Promise::new(env::predecessor_account_id()).transfer(refund);
            }
        
        }
        
    }
    
    //get free playses in the tournament  
    fn display_freeplaces_in_tournament(&self, tournament_id: TournamentId) -> Option<U64> {
                
        //if there is some tournament ID in the tournaments_by_id collection
        if let Some(_tournament) = self.tournaments_by_id.get(&tournament_id) {
        
            let metadata = self.tournament_metadata_by_id.get(&tournament_id).unwrap();
            
            //calculate free places
            let free_places = metadata.players_number-self.internal_get_players_number_in_tournament(&tournament_id);
            
            //return free places
            Some((free_places as u64).into())
            
        } else { //if there wasn't a tournament_id ID in the tournaments_by_id collection, we return None
            None
        }        
        
    }
    
    //refunds the prizes for the winners 
    fn reward_prizes(&mut self, tournament_id: TournamentId, winners_map: HashMap<u8,AccountId>)  {
        
        //if there is some tournament ID in the tournaments_by_id collection
        if let Some(mut tournament) = self.tournaments_by_id.get(&tournament_id) {
            
            //check the owner calls this method
            assert_eq!(env::predecessor_account_id(), tournament.owner_id, "Owner's method");
            
            //check the tournament is active
            assert!(tournament.active, "Tournament is inactive");
            
            
            //get prizes values in persent for the places
            let prizes_map = self.winners_percents_per_tournament.get(&tournament_id).unwrap();
            
            //summarize the rewards
            let mut sum_reward = 0;
            
            //reward prizes
            for (place,account) in winners_map {
            
                let percents: u128 = prizes_map.get(&place).unwrap().into();
                
                //calculate the percents of the prize fond
                let reward_amount = percent_calculation(&percents, &tournament.balance);
                
                //refund the prize
                Promise::new(account).transfer(reward_amount);
                
                //summarize the rewards
                sum_reward+=reward_amount;
            }
            
            //decrease the prize fond of tournament 
            tournament.balance-=sum_reward;
            
            //inactivate the tournament
            tournament.active=false;
            
            self.tournaments_by_id.insert(&tournament_id, &tournament);
            
        }         
        
    }
    
}

//calculates the percents from the amount
fn percent_calculation ( &percent_value: &u128, &amount: &u128)-> u128 {
    
    let mut percent_amount: u128 = (percent_value * amount)/100;
    let reminder = (percent_value * amount)%100;
                
        if reminder!=0{
            if ((amount * 10 )/reminder) > 4 {
                percent_amount+=1;
            } 
        }
        
    return percent_amount;        
}



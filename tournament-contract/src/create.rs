use crate::*;

#[near_bindgen]
impl Contract {
    //tournament creation method
    pub fn tournament_create(
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
        
        assert!(u128::from(in_price)>0,"Tournaments with zero in prise are not allowed");
        
        //specify the tornament struct that contains the owner ID 
        let tournament = Tournament {
            //set the owner ID equal to the tournament owner ID passed into the function
            owner_id: tournament_owner_id,
            active: true,
            balance: 0,            
        };

        //insert the tornament ID and tournament struct and make sure that the tournament doesn't exist
        assert!(
            self.tournaments_by_id.insert(&tournament_id, &tournament).is_none(),
            "Tornament already exists"
        );
        
        //specify the tournament metadata struct
        let metadata = TournamentMetadata {
            name: name,                
            icon: icon,
            players_number: players_number,
            in_price: u128::from(in_price),
        };

        //insert the tornament ID and metadata
        self.tournament_metadata_by_id.insert(&tournament_id, &metadata);
        
        //insert the prizes percents for the tournament ID
        self.internal_add_prizes_to_tournament(&tournament_id, &percents_map);
    }
}

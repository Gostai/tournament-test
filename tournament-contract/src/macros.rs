/// The core methods for a basic tournament. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_tournament_contract_core {
    ($contract: ident, $tournament: ident) => {
        use $crate::tournament::tournament_core::TournamentContractCore;
        
        #[near_bindgen]
        impl TournamentContractCore for $contract {
            fn tournament_create(
                &mut self,
                tournament_id: TournamentId,
                name: String,
                icon: Option<String>,
                players_number: u8,
                in_price: U128,        
                tournament_owner_id: AccountId,
                percents_map: HashMap<u8,u8>,
            ) {
                self.$tournament.tournament_create(tournament_id, name, icon, players_number, in_price, tournament_owner_id, percents_map)
            }
        
            fn display_tournament(
                &self,
                tournament_id: TournamentId
            ) -> Option<JsonTournament> {
                self.$tournament.display_tournament(tournament_id)
            }
            
            #[payable]
            fn participate_tournament(
                &mut self,
                tournament_id: TournamentId
            ) {
                self.$tournament.participate_tournament(tournament_id)
            }    
            
            fn display_freeplaces_in_tournament(
                &self,
                tournament_id: TournamentId
            ) -> Option<U64> {
                self.$tournament.display_freeplaces_in_tournament(tournament_id)
            } 
            
            fn reward_prizes(
                &mut self, 
                tournament_id: TournamentId, 
                winners_map: HashMap<u8,AccountId>
            ) {
                self.$tournament.reward_prizes(tournament_id, winners_map)
            }
        }
    };
}

/// Tournament enumeration adds the extension standard offering 
/// view-only methods.
#[macro_export]
macro_rules! impl_tournament_contract_enumeration {
    ($contract: ident, $tournament: ident) => {
        use $crate::tournament::enumeration::TournamentContractEnumeration;

        #[near_bindgen]
        impl TournamentContractEnumeration for $contract {
            
            fn display_tournaments(
                &self, 
                from_index: Option<U128>,
                limit: Option<u64>
            ) -> Vec<JsonTournament> {
                self.$tournament.display_tournaments(from_index, limit)
            }   
        }
    };
}

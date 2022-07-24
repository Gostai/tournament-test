use crate::event::NearEvent;
use near_sdk::{AccountId};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Serialize};

/// Enum that represents the data type of the EventLog.
#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
#[serde(crate = "near_sdk::serde")]
pub enum EventLogVariant<'a> {
    TournamentCreate(&'a [TournamentCreateLog<'a>]),
    TournamentEntrance(&'a [TournamentEntranceLog<'a>]),
    TournamentPrizesReward(&'a [TournamentPrizesRewardLog<'a>]),
}

/// Interface to capture data about an event
///
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub(crate) struct EventLog<'a> {    
    pub version: &'static str,
    
    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: EventLogVariant<'a>,
}

/// An event log to capture tournament creation
///
/// Arguments
/// * `tournament_id`: "tournament-1" 
/// * `players_number`: 8
/// * `in_price`: "100000"
#[must_use]
#[derive(Serialize,  Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentCreateLog<'a> {    
    pub tournament_id: &'a String,
    pub players_number: &'a u8,
    pub in_price:&'a U128,
}

impl TournamentCreateLog<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }
    
    pub fn emit_many(data: &[TournamentCreateLog<'_>]) {
        new_mf1_v1(EventLogVariant::TournamentCreate(data)).emit()
    }
}

/// An event log to capture tournament entrance
///
/// Arguments
/// * `partisipator_id`: "partisipator.near"
/// * `tournament_id`: "tournament-1"
#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentEntranceLog<'a> { 
    pub partisipator_id:&'a AccountId,
    pub tournament_id:&'a String,    
} 

impl TournamentEntranceLog<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }
    
    pub fn emit_many(data: &[TournamentEntranceLog<'_>]) {
        new_mf1_v1(EventLogVariant::TournamentEntrance(data)).emit()
    }
}


/// An event log to capture tournament prize rewarding
///
/// Arguments
/// * `tournament_id`: "tournament-1"
/// * `rewarded_amount`: "100000000"
#[derive(Serialize,  Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentPrizesRewardLog<'a> {     
    pub tournament_id:&'a String,    
    pub rewarded_amount:&'a u128,
} 

impl TournamentPrizesRewardLog<'_> {
    pub fn emit(self) {
        Self::emit_many(&[self])
    }
    
    pub fn emit_many(data: &[TournamentPrizesRewardLog<'_>]) {
        new_mf1_v1(EventLogVariant::TournamentPrizesReward(data)).emit()
    }
}

fn new_mf1<'a>(version: &'static str, event: EventLogVariant<'a>) -> NearEvent<'a> {
  NearEvent::Mf1(EventLog { version, event })
}

fn new_mf1_v1(event: EventLogVariant) -> NearEvent {
  new_mf1("1.0.0", event)
}

use std::time::Duration;

use crate::events::{EventsToListenActions, PlayerList, PlayerListActions};

mod action;
mod events;
mod requests;

pub fn main() {
    loop {
        println!("Waiting for match...");
        let players;
        loop {
            players = match PlayerList::get() {
                None => {
                    std::thread::sleep(Duration::new(2, 0));
                    continue;
                }
                Some(p) => p,
            };
            break;
        }
        let players_to_observe = players.pick_players_to_observe();
        let mut players_events_to_observe = Vec::new();
        players_events_to_observe.create_events_for_players(players_to_observe);
        events::listen(players_events_to_observe);
    }
}

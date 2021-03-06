use std::time::Duration;

use action::get_actions;

use crate::events::{EventsToListenActions, PlayerList, PlayerListActions};

mod action;
mod events;
mod requests;

pub fn main() {
    let available_actions = get_actions();
    loop {
        println!("Waiting for a match...");
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
        players_events_to_observe.create_events_for_players(players_to_observe, &available_actions);
        events::listen(players_events_to_observe);
    }
}

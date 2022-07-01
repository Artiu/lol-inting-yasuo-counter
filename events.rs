use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use serde::Deserialize;
use serde_json::Value;

use crate::{action, requests};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    champion_name: String,
    summoner_name: String,
}

pub type PlayerList = Vec<Player>;

pub trait PlayerListActions {
    fn get() -> Option<PlayerList>;
    fn pick_players_to_observe(&self) -> Self;
}

impl PlayerListActions for PlayerList {
    fn get() -> Option<Self> {
        requests::get_league_live_data::<Self>("/playerlist")
    }
    fn pick_players_to_observe(&self) -> Self {
        for i in 0..self.len() {
            let player = &self[i];
            println!(
                "{}.Nickname: {}, Champion: {}",
                i + 1,
                player.summoner_name,
                player.champion_name
            );
        }
        println!("Pick players to make action on: ");
        'main_loop: loop {
            let mut user_input = String::new();
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Problem with reading user input!");

            let splited_input: HashSet<&str> = user_input.trim().split(" ").collect();
            let mut players: PlayerList = Vec::new();
            for input_part in splited_input {
                match input_part.parse::<usize>() {
                    Err(_) => {
                        println!("Incorrect input!");
                        continue 'main_loop;
                    }
                    Ok(parsed) => {
                        if parsed < 1 || parsed > self.len() {
                            println!("One of provided players doesn't exist");
                            continue 'main_loop;
                        }
                        players.push(self[parsed - 1].clone())
                    }
                };
            }
            return players;
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EventResponse {
    events: Vec<HashMap<String, Value>>,
}

fn get_events() -> Option<Vec<HashMap<String, Value>>> {
    match requests::get_league_live_data::<EventResponse>("/eventdata") {
        Some(e) => Some(e.events),
        None => None,
    }
}

#[derive(PartialEq, Debug)]
enum Event {
    Kill,
    Death,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct EventToListen {
    player: Player,
    event: Event,
    action: action::Action,
}

pub type EventsToListen = Vec<EventToListen>;

pub trait EventsToListenActions {
    fn create_events_for_players(&mut self, players: PlayerList);
}

impl EventsToListenActions for EventsToListen {
    fn create_events_for_players(&mut self, players: PlayerList) {
        players.iter().for_each(|player| {
            println!(
                "Pick event for {} ({}): ",
                player.summoner_name, player.champion_name
            );
            println!("1. Death");
            println!("2. Kill");
            let user_event;
            loop {
                let mut user_choice = String::new();
                std::io::stdin().read_line(&mut user_choice).unwrap();
                match user_choice.trim() {
                    "1" => user_event = Event::Death,
                    "2" => user_event = Event::Kill,
                    _ => {
                        println!("Incorrect option!");
                        continue;
                    }
                }
                break;
            }
            println!("Pick action: ");
            println!("1. Show mastery emote");
            println!("2. Show random emote from wheel");
            let user_action;
            loop {
                let mut user_choice = String::new();
                std::io::stdin().read_line(&mut user_choice).unwrap();
                match user_choice.trim() {
                    "1" => user_action = action::Action::MasteryEmote,
                    "2" => user_action = action::Action::RandomEmote,
                    _ => {
                        println!("Incorrect option!");
                        continue;
                    }
                }
                break;
            }
            self.push(EventToListen {
                player: player.clone(),
                event: user_event,
                action: user_action,
            });
        });
    }
}

pub fn listen(events_to_listen: Vec<EventToListen>) {
    let mut events = match get_events() {
        Some(events) => events,
        None => return,
    };
    let mut last_event_length = events.len();
    println!("Listening on events...");
    loop {
        events = match get_events() {
            Some(events) => events,
            None => break,
        };

        if last_event_length < events.len() {
            for event in &events[last_event_length..events.len()] {
                let name = event.get("EventName").unwrap();
                if name != "ChampionKill" {
                    continue;
                }
                let dead_player_name = event.get("VictimName").unwrap();
                let killer_player_name = event.get("KillerName").unwrap();
                let event = events_to_listen.iter().find(|e| {
                    (e.player.summoner_name.as_str() == dead_player_name && e.event == Event::Death)
                        || (e.player.summoner_name.as_str() == killer_player_name
                            && e.event == Event::Kill)
                });
                if let Some(e) = event {
                    println!(
                        "{} ({}): {}",
                        e.player.summoner_name, e.player.champion_name, e.event
                    );
                    e.action.make();
                }
            }
            last_event_length = events.len();
        }
        std::thread::sleep(std::time::Duration::new(1, 0));
    }
}

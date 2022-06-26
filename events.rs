use serde::Deserialize;

use crate::actions;

#[derive(Deserialize)]
struct Scores {
    deaths: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Champion {
    champion_name: String,
    summoner_name: String,
    scores: Scores,
}

fn get_league_live_data<T: for<'de> serde::Deserialize<'de>>(path: &str) -> T {
    let url = format!("{}{}", "https://127.0.0.1:2999/liveclientdata", path);
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    loop {
        return match client.get(&url).send() {
            Ok(res) => res.json().unwrap(),
            _ => {
                println!("Problem with League of Legends live data connection! Next reconnection attempt will be in 30 seconds");
                std::thread::sleep(std::time::Duration::new(30, 0));
                continue;
            }
        };
    }
}

fn listen_on_champion_death(name: &str) {
    let name_string = name.to_string();
    let champions: Vec<Champion> = get_league_live_data("/playerlist");
    let champion_player = match champions
        .iter()
        .find(|champion| champion.champion_name == name_string)
    {
        Some(champion) => champion,
        None => {
            println!("You don't have {} in game!", name_string);
            return;
        }
    };
    let mut champion_deaths = champion_player.scores.deaths;
    loop {
        let url = format!(
            "{}{}",
            "/playerscores?summonerName=", champion_player.summoner_name
        );
        let champion_stats: Scores = get_league_live_data(&url);
        if champion_stats.deaths > champion_deaths {
            actions::show_mastery_emote();
            champion_deaths = champion_stats.deaths;
        }
        std::thread::sleep(std::time::Duration::new(1, 0));
    }
}

pub fn listen() {
    listen_on_champion_death("Yasuo");
}

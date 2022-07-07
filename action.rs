use std::{collections::HashMap, fs, path::Path};

use enigo::*;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Action {
    pub name: String,
    key: char,
    #[serde(default = "default_with_ctrl")]
    with_ctrl: bool,
}

fn default_with_ctrl() -> bool {
    false
}

impl Action {
    pub fn make(&self) {
        let mut enigo = Enigo::new();
        match self.with_ctrl {
            false => {
                enigo.key_click(Key::Layout(self.key));
            }
            true => {
                enigo.key_down(Key::Control);
                enigo.key_click(Key::Layout(self.key));
                enigo.key_up(Key::Control);
            }
        }
        println!("Showed {}!", self.name);
    }
}

fn get_default_actions() -> Vec<Action> {
    vec![Action {
        name: "mastery emote".to_string(),
        key: '6',
        with_ctrl: true,
    }]
}

pub fn get_actions() -> Vec<Action> {
    let config = match fs::read_to_string(Path::new("config.json")) {
        Err(_) => return get_default_actions(),
        Ok(cfg) => cfg,
    };
    let parsed_config: HashMap<String, Vec<Action>> =
        serde_json::from_str(&config).expect("Incorrect config file!");
    let actions = match parsed_config.get("actions") {
        None => panic!("Incorrect config!"),
        Some(a) => a,
    };
    if actions.len() == 0 {
        panic!("Actions must have at least one element");
    }
    actions.to_vec()
}

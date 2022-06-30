use std::fmt::Display;

use enigo::*;

#[derive(Clone, Debug)]
pub enum Action {
    MasteryEmote,
    LikeEmote,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Action::MasteryEmote => write!(f, "mastery emote"),
            Action::LikeEmote => write!(f, "like emote"),
        }
    }
}

impl Action {
    pub fn make(&self) {
        let mut enigo = Enigo::new();
        match self {
            Action::MasteryEmote => {
                enigo.key_down(Key::Control);
                enigo.key_click(Key::Layout('6'));
                enigo.key_up(Key::Control);
                println!("Showed mastery emote!");
            }
            Action::LikeEmote => {
                enigo.key_click(Key::Layout('T'));
                println!("Showed like emote!");
            }
        }
    }
}

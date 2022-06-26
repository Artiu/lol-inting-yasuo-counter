use enigo::*;

pub fn show_mastery_emote() {
    let mut enigo = Enigo::new();
    println!("Showed mastery emote!");
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('6'));
    enigo.key_up(Key::Control);
}

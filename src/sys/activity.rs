use enigo::{Enigo, Key, MouseControllable, KeyboardControllable};
use std::{thread, time::Duration};

pub fn simulate_activity() -> Result<(), Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new();

    loop {
        enigo.mouse_move_relative(1, 1);
        enigo.mouse_move_relative(-1, -1);

        enigo.key_down(Key::Shift);
        enigo.key_up(Key::Shift);

        thread::sleep(Duration::from_secs(180)); // TODO: make this configurable
    }
}

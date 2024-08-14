use crate::ihm::debug_console::debug_console::DebugConsole;
use crate::model::game::Game;
use std::sync::mpsc;
use std::thread;

pub mod model;

pub mod ihm;
fn main() {
    let (tx_command, rx_command) = mpsc::channel();

    let mut game = Game::default();

    let mut debug_console = DebugConsole::new(tx_command.clone());

    game.subscribe(debug_console.get_channel_listener());

    let game_handle = thread::spawn(move || loop {
        let exit = match rx_command.recv() {
            Ok(game_command) => game.execute(game_command),
            Err(_) => false,
        };

        if exit {
            break;
        }
    });

    debug_console.start();

    game_handle.join().unwrap();
}

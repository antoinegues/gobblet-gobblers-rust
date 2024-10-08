use crate::ihm::channel_listener::ChannelListener;
use crate::model::game_command::GameCommand;
use crate::model::game_event::GameEvent;
use crate::model::piece_size::PieceSize::{Big, Medium, Small};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::{io, thread};
use crate::model::piece_size::PieceSize;

pub struct DebugConsole {
    rx_event: Option<Receiver<GameEvent>>,
    listener: Arc<ChannelListener>,
    tx_command: Sender<GameCommand>,
}

fn piece_size_from_number(i: &str) -> Result<PieceSize, ()> {
    match i {
        "1" => Ok(Small),
        "2" => Ok(Medium),
        "3" => Ok(Big),
        _ => Err(()),
    }



}

impl DebugConsole {
    pub(crate) fn new(tx_command: Sender<GameCommand>) -> Self {
        let (tx, rx) = mpsc::channel();

        DebugConsole {
            rx_event: Some(rx),
            listener: Arc::from(ChannelListener::new(tx)),
            tx_command,
        }
    }

    pub fn get_channel_listener(&self) -> Arc<ChannelListener> {
        return Arc::clone(&self.listener);
    }

    pub fn start(&mut self) {
        let tx_command = self.tx_command.clone();
        thread::spawn(move || loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Échec de la lecture de l'entrée utilisateur");

            match &input.as_str()[0..1] {
                "1" => tx_command
                    .send(GameCommand::NewGameCommand)
                    .expect("Erreur lors de l'envoie de la commande"),
                "2" => tx_command
                    .send(GameCommand::PutPieceCommand(
                        input[2..3].parse().unwrap(),
                        input[4..5].parse().unwrap(),
                        piece_size_from_number(&input[6..7]).expect("Impossible de convertir"),
                    ))
                    .expect("Erreur lors de l'envoie de la commande"),
                "3" => tx_command
                    .send(GameCommand::MovePieceCommand(
                        input[2..3].parse().unwrap(),
                        input[4..5].parse().unwrap(),
                        input[6..7].parse().unwrap(),
                        input[8..9].parse().unwrap(),
                    ))
                    .expect("Erreur lors de l'envoie de la commande"),
                "4" => tx_command
                    .send(GameCommand::ExitCommand)
                    .expect("Erreur lors de l'envoie de la commande"),
                _ => {
                    println!("Commande inconnue : {}", &input)
                }
            };
        });

        let rx_event = self.rx_event.take().unwrap();
        thread::spawn(move || loop {
            match rx_event.recv() {
                Ok(game_event) => println!("{:#?}", game_event),
                Err(_) => {}
            }
        });
    }
}

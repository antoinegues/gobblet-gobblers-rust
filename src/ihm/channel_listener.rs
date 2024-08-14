use crate::model::game_event::GameEvent;
use crate::model::listener::Listener;
use std::sync::mpsc::Sender;

pub struct ChannelListener {
    tx: Sender<GameEvent>,
}

impl ChannelListener {
    pub(crate) fn new(tx: Sender<GameEvent>) -> ChannelListener {
        ChannelListener { tx }
    }
}

impl Listener for ChannelListener {
    fn notify(&self, game_event: GameEvent) {
        self.tx
            .send(game_event)
            .expect("Impossible de transmettre l'event");
    }
}

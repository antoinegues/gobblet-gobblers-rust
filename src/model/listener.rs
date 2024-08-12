use crate::model::game::GameEvent;

pub trait Listener {
    fn notify(&self, game_event: GameEvent);
}
use super::game_event::GameEvent;

pub trait Listener: Send + Sync {
    fn notify(&self, game_event: GameEvent);
}
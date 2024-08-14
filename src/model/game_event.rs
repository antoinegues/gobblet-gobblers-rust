use crate::model::game_error::GameError;
use crate::model::game_state::GameState;
use crate::model::player::Color;

#[derive(Clone, Debug)]
pub enum GameEvent {
    NewGameEvent(GameState),
    BoardUpdateEvent(GameState),
    GameWinEvent(Color),
    GameErrorEvent(GameError),
    ExitEvent,
}

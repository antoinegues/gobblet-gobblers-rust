use crate::model::game::GameCommand::{NewGameCommand, PutPieceCommand};
use crate::model::game::GameEvent::{BoardUpdateEvent, GameErrorEvent, NewGameEvent};
use crate::model::game_error::GameError;
use crate::model::game_error::GameError::{CurrentlyNoGame, UnknownError};
use crate::model::game_instance::GameInstance;
use crate::model::game_state::GameState;
use crate::model::listener::Listener;
use crate::model::piece::PieceSize;
use crate::model::player::Color;

pub struct Game<'a> {
    game_instance: Option<GameInstance>,
    listeners: Vec<Box<&'a dyn Listener>>,
}
impl Default for Game<'_> {
    fn default() -> Self {
        Game {
            game_instance: None,
            listeners: Vec::new(),
        }
    }
}

impl<'a> Game<'a> {
    fn new_game(&mut self) -> Result<GameState, GameError> {
        self.game_instance = Some(GameInstance::default());
        match &self.game_instance {
            Some(game_instance) => Ok(game_instance.to_game_state()),
            _ => Err(UnknownError),
        }
    }

    fn put_piece(
        &mut self,
        x: usize,
        y: usize,
        piece_size: PieceSize,
    ) -> Result<GameState, GameError> {
        match self.game_instance {
            Some(ref mut game_instance)  => {
                game_instance.put_piece(x, y, piece_size)?;
                Ok(game_instance.to_game_state())
            },
            None => Err(CurrentlyNoGame(String::from("Il n'y a aucune partie en cours"))),
        }
    }

    pub fn execute(&mut self, game_command: GameCommand) {
        let command_result = match game_command {
            NewGameCommand => self.new_game().map(|game_state| NewGameEvent(game_state)),
            PutPieceCommand(x, y, size) => self.put_piece(x, y, size).map(|game_state| BoardUpdateEvent(game_state)),
            _ => Err(UnknownError),
        };

        let event = command_result.unwrap_or_else(|game_error| GameErrorEvent(game_error));
        self.notify_all(event);
    }

    pub fn subscribe<'b>(&'b mut self, listener: &'a dyn Listener) {
        self.listeners.push(Box::from(listener));
    }

    fn notify_all(&self, game_event: GameEvent) {
        for listener in self.listeners.as_slice() {
            listener.notify(game_event.clone());
        }
    }
}

pub enum GameCommand {
    NewGameCommand,
    PutPieceCommand(usize, usize, PieceSize),
    MovePieceCommand(usize, usize, usize, usize),
}

#[derive(Clone)]
pub enum GameEvent {
    NewGameEvent(GameState),
    BoardUpdateEvent(GameState),
    GameWinEvent(Color),
    GameErrorEvent(GameError),
}

#[cfg(test)]
mod tests {
    use crate::model::game::GameCommand::{NewGameCommand, PutPieceCommand};
    use crate::model::game::{Game, GameEvent};
    use crate::model::listener::Listener;
    use std::cell::RefCell;
    use crate::model::game::GameEvent::{BoardUpdateEvent, NewGameEvent};
    use crate::model::piece::PieceSize::Small;

    struct GameEventListenerMock {
        last_event: RefCell<Option<GameEvent>>,
    }

    impl Listener for GameEventListenerMock {
        fn notify(&self, game_event: GameEvent) {
            let mut last_event = self.last_event.borrow_mut();
            *last_event = Some(game_event);
        }
    }

    #[test]
    fn game_new_test() {
        let game = Game::default();
        assert!(game.game_instance.is_none());
        assert_eq!(game.listeners.len(), 0);
    }

    #[test]
    fn new_game_command() {
        let mut game = Game::default();

        assert!(game.game_instance.is_none());

        game.execute(NewGameCommand);

        assert!(game.game_instance.is_some());
    }

    #[test]
    fn new_game_event() -> Result<(), ()> {
        let mut game = Game::default();
        let game_listener = GameEventListenerMock {
            last_event: RefCell::new(None),
        };

        game.subscribe(&game_listener);

        game.execute(NewGameCommand);

        let last_event = game_listener.last_event.borrow_mut().take();

        match last_event {
            Some(event) => match event {
                NewGameEvent(_) => Ok(()),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    #[test]
    fn put_piece_command_test() -> Result<(),()> {
        let mut game = Game::default();

        let game_listener = GameEventListenerMock {
            last_event: RefCell::new(None),
        };

        game.subscribe(&game_listener);

        game.execute(NewGameCommand);


        let game_event = game_listener.last_event.borrow_mut().take();
        game.execute(PutPieceCommand(0, 0, Small));

        let event = match game_listener.last_event.borrow_mut().take() {
            Some(event) => event,
            _ => return Err(()),
        };

        match event {
            BoardUpdateEvent(game_state) => match game_state.board.squares[0][0] {
                Some(_) => Ok(()),
                None => Err(()),
            },
            _ => Err(()),
        }
    }
}

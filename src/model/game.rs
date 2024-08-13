use std::sync::Arc;
use crate::model::game::GameCommand::{NewGameCommand, PutPieceCommand};
use crate::model::game::GameEvent::{BoardUpdateEvent, GameErrorEvent, NewGameEvent};
use crate::model::game_command::GameCommand;
use crate::model::game_error::GameError;
use crate::model::game_error::GameError::{CurrentlyNoGame, UnknownError};
use crate::model::game_event::GameEvent;
use crate::model::game_instance::GameInstance;
use crate::model::game_state::GameState;
use crate::model::listener::Listener;
use crate::model::piece_size::PieceSize;

pub struct Game {
    game_instance: Option<GameInstance>,
    listeners: Vec<Box<Arc<dyn Listener>>>,
}
impl Default for Game {
    fn default() -> Self {
        Game {
            game_instance: None,
            listeners: Vec::new(),
        }
    }
}

impl Game {
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

    pub fn subscribe<'b>(&'b mut self, listener: Arc<dyn Listener>) {
        self.listeners.push(Box::from(listener));
    }

    fn notify_all(&self, game_event: GameEvent) {
        for listener in self.listeners.as_slice() {
            listener.notify(game_event.clone());
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::sync::Arc;
    use crate::model::game::Game;
    use crate::model::game_command::GameCommand::{NewGameCommand, PutPieceCommand};
    use crate::model::game_event::GameEvent;
    use crate::model::game_event::GameEvent::{BoardUpdateEvent, NewGameEvent};
    use crate::model::listener::Listener;
    use crate::model::piece_size::PieceSize::Small;

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

        game.subscribe(Arc::from(game_listener));

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

        game.subscribe(Arc::from(game_listener));

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

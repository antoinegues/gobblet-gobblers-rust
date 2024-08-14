use crate::model::game::GameCommand::*;
use crate::model::game::GameEvent::{BoardUpdateEvent, GameErrorEvent, NewGameEvent};
use crate::model::game_command::GameCommand;
use crate::model::game_command::GameCommand::MovePieceCommand;
use crate::model::game_error::GameError;
use crate::model::game_error::GameError::{CurrentlyNoGame, UnknownError};
use crate::model::game_event::GameEvent;
use crate::model::game_event::GameEvent::ExitEvent;
use crate::model::game_instance::GameInstance;
use crate::model::game_state::GameState;
use crate::model::listener::Listener;
use crate::model::piece_size::PieceSize;
use std::sync::Arc;

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
            Some(ref mut game_instance) => {
                game_instance.put_piece(x, y, piece_size)?;
                Ok(game_instance.to_game_state())
            }
            None => Err(CurrentlyNoGame(String::from(
                "Il n'y a aucune partie en cours",
            ))),
        }
    }

    pub fn move_piece(
        &mut self,
        origin_x: usize,
        origin_y: usize,
        destination_x: usize,
        destination_y: usize,
    ) -> Result<GameState, GameError> {
        match self.game_instance {
            Some(ref mut game_instance) => {
                game_instance.move_piece(origin_x, origin_y, destination_x, destination_y)?;
                Ok(game_instance.to_game_state())
            }
            None => Err(CurrentlyNoGame(String::from(
                "Il n'y a aucune partie en cours",
            ))),
        }
    }

    pub fn execute(&mut self, game_command: GameCommand) -> bool {
        let command_result = match game_command {
            NewGameCommand => self.new_game().map(|game_state| NewGameEvent(game_state)),
            PutPieceCommand(x, y, size) => self
                .put_piece(x, y, size)
                .map(|game_state| BoardUpdateEvent(game_state)),
            MovePieceCommand(origin_x, origin_y, destination_x, destination_y) => self
                .move_piece(origin_x, origin_y, destination_x, destination_y)
                .map(|game_state| BoardUpdateEvent(game_state)),
            ExitCommand => Ok(ExitEvent),
        };

        let event = command_result.unwrap_or_else(|game_error| GameErrorEvent(game_error));
        self.notify_all(event.clone());

        match event {
            ExitEvent => true,
            _ => false,
        }
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
    use crate::model::game::Game;
    use crate::model::game_command::GameCommand::{
        ExitCommand, MovePieceCommand, NewGameCommand, PutPieceCommand,
    };
    use crate::model::game_event::GameEvent;
    use crate::model::game_event::GameEvent::{BoardUpdateEvent, ExitEvent, NewGameEvent};
    use crate::model::listener::Listener;
    use crate::model::piece_size::PieceSize::{Medium, Small};
    use crate::model::player::Color::Red;
    use std::sync::{Arc, Mutex};

    struct GameEventListenerMock {
        last_event: Mutex<Option<GameEvent>>,
    }

    impl Listener for GameEventListenerMock {
        fn notify(&self, game_event: GameEvent) {
            let mut last_event = self.last_event.lock().unwrap();
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
            last_event: Mutex::new(None),
        };

        let arc = Arc::from(game_listener);
        game.subscribe(Arc::clone(&arc) as Arc<dyn Listener>);

        game.execute(NewGameCommand);

        let last_event = arc.last_event.lock().unwrap().take();

        match last_event {
            Some(event) => match event {
                NewGameEvent(_) => Ok(()),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    #[test]
    fn put_piece_command_test() -> Result<(), ()> {
        let mut game = Game::default();

        let game_listener = GameEventListenerMock {
            last_event: Mutex::new(None),
        };

        let arc = Arc::from(game_listener);
        game.subscribe(Arc::clone(&arc) as Arc<dyn Listener>);

        game.execute(NewGameCommand);

        game.execute(PutPieceCommand(0, 0, Small));

        let last_event = arc.last_event.lock().unwrap().take();
        let event = match last_event {
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

    #[test]
    fn move_piece_command_test() -> Result<(), ()> {
        let mut game = Game::default();

        let game_listener = GameEventListenerMock {
            last_event: Mutex::new(None),
        };

        let arc = Arc::from(game_listener);
        game.subscribe(Arc::clone(&arc) as Arc<dyn Listener>);

        game.execute(NewGameCommand);

        game.execute(PutPieceCommand(0, 0, Medium));

        game.execute(PutPieceCommand(1, 1, Small));

        game.execute(MovePieceCommand(0, 0, 1, 1));

        let last_event = arc.last_event.lock().unwrap().take();
        let event = match last_event {
            Some(event) => event,
            _ => return Err(()),
        };

        match event {
            BoardUpdateEvent(game_state) => match &game_state.board.squares[1][1] {
                Some(piece) => {
                    if piece.size == Medium && piece.color == Red {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                None => Err(()),
            },
            _ => Err(()),
        }
    }

    #[test]
    fn exit_command_test() -> Result<(), ()> {
        let mut game = Game::default();
        let game_listener = GameEventListenerMock {
            last_event: Mutex::new(None),
        };

        let arc = Arc::from(game_listener);
        game.subscribe(Arc::clone(&arc) as Arc<dyn Listener>);

        let exit = game.execute(NewGameCommand);
        assert!(!exit);

        let exit = game.execute(ExitCommand);

        assert!(exit);
        let last_event = arc.last_event.lock().unwrap().take();

        match last_event {
            Some(event) => match event {
                ExitEvent => Ok(()),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

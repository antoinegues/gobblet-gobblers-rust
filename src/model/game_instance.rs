use crate::model::board::Board;
use crate::model::game_error::GameError;
use crate::model::game_error::GameError::{NotYourPiece, SquareIsEmpty};
use crate::model::game_state::GameState;
use crate::model::piece_size::PieceSize;
use crate::model::player::Color::{Blue, Red};
use crate::model::player::{Color, Player};

pub struct GameInstance {
    board: Board,
    players: [Player; 2],
    turn: u32,
}

impl Default for GameInstance {
    fn default() -> Self {
        GameInstance {
            board: Board::default(),
            players: [Player::new(Red), Player::new(Blue)],
            turn: 0,
        }
    }
}

impl GameInstance {
    pub fn next_turn(&mut self) {
        self.turn += 1;
    }

    pub fn get_current_player(&mut self) -> &mut Player {
        return if self.turn % 2 == 0 {
            &mut self.players[0]
        } else {
            &mut self.players[1]
        };
    }

    pub fn put_piece(
        &mut self,
        x: usize,
        y: usize,
        piece_size: PieceSize,
    ) -> Result<(), GameError> {
        let current_player = self.get_current_player();
        let piece = current_player.remove_piece(piece_size)?;
        self.board.put_piece(x, y, piece)?;
        self.next_turn();
        Ok(())
    }

    pub fn move_piece(
        &mut self,
        origin_x: usize,
        origin_y: usize,
        destination_x: usize,
        destination_y: usize,
    ) -> Result<(), GameError> {
        let piece_color = self
            .board
            .get_piece_color(origin_x, origin_y)
            .ok_or_else(|| {
                SquareIsEmpty(String::from(
                    "Impossible de bouger la pièce, la case est vide",
                ))
            })?;
        let current_player = self.get_current_player();

        if piece_color != current_player.color {
            return Err(NotYourPiece(String::from(
                "Vous ne pouvez pas bouger une pièce qui ne vous appartient pas",
            )));
        }

        self.board
            .move_piece(origin_x, origin_y, destination_y, destination_x)?;
        self.next_turn();
        Ok(())
    }

    pub fn get_piece_size(&self, x: usize, y: usize) -> Option<PieceSize> {
        self.board.get_piece_size(x,y)
    }

    pub fn get_piece_color(&self, x: usize, y: usize) -> Option<Color> {
        self.board.get_piece_color(x,y)
    }

    pub fn to_game_state(&self) -> GameState {
        GameState {
            players : [self.players[0].to_player_state(), self.players[1].to_player_state()],
            board: self.board.to_board_state(),
            turn: self.turn,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::game_error::GameError::{NotYourPiece, PieceNotAvailable, SquareIsEmpty};
    use crate::model::game_instance::GameInstance;
    use crate::model::piece_size::PieceSize::{Big, Medium, Small};
    use crate::model::player::Color::{Blue, Red};

    #[test]
    fn game_instance_new_test() {
        let game_instance = GameInstance::default();
        assert_eq!(game_instance.players[0].color, Red);
        assert_eq!(game_instance.players[1].color, Blue);
        assert_eq!(game_instance.turn, 0);
    }

    #[test]
    fn next_turn_test() {
        let mut game_instance = GameInstance::default();

        assert_eq!(game_instance.turn, 0);

        game_instance.next_turn();

        assert_eq!(game_instance.turn, 1);
    }

    #[test]
    fn get_current_player_test() {
        let mut game_instance = GameInstance::default();

        assert_eq!(game_instance.get_current_player().color, Red);

        game_instance.next_turn();

        assert_eq!(game_instance.get_current_player().color, Blue);
    }

    #[test]
    fn put_piece_player_switch_test() {
        let mut game_instance = GameInstance::default();

        game_instance
            .put_piece(1, 1, Small)
            .expect("Impossible de placer la pièce");

        assert_eq!(game_instance.get_piece_color(1, 1).unwrap(), Red);

        game_instance
            .put_piece(1, 1, Medium)
            .expect("Impossible de placer la pièce");

        assert_eq!(game_instance.get_piece_color(1, 1).unwrap(), Blue);
    }

    #[test]
    fn put_piece_player_switch_with_error_test() {
        let mut game_instance = GameInstance::default();

        game_instance
            .put_piece(1, 1, Medium)
            .expect("Impossible de placer la pièce");

        assert_eq!(game_instance.get_piece_color(1, 1).unwrap(), Red);

        assert!(game_instance.put_piece(1, 1, Medium).is_err());

        game_instance
            .put_piece(1, 1, Big)
            .expect("Impossible de placer la pièce");

        assert_eq!(game_instance.get_piece_color(1, 1).unwrap(), Blue);
    }

    #[test]
    fn move_piece_test() {
        let mut game_instance = GameInstance::default();

        game_instance
            .put_piece(1, 1, Medium)
            .expect("Impossible de placer la pièce");

        game_instance
            .put_piece(2, 2, Small)
            .expect("Impossible de placer la pièce");

        game_instance
            .move_piece(1, 1, 2, 2)
            .expect("Impossible de déplacer la pièce");

        assert_eq!(game_instance.get_piece_size(2,2).unwrap(), Medium);
        assert_eq!(game_instance.get_piece_color(2,2).unwrap(), Red);
    }

    #[test]
    fn move_piece_player_switch_test() {
        let mut game_instance = GameInstance::default();

        game_instance
            .put_piece(1, 1, Medium)
            .expect("Impossible de placer la pièce");

        game_instance
            .put_piece(2, 2, Small)
            .expect("Impossible de placer la pièce");

        game_instance
            .move_piece(1, 1, 2, 2)
            .expect("Impossible de déplacer la pièce");


        game_instance.put_piece(1,1, Big).expect("Impossible de placer la pièce");

        assert_eq!(game_instance.get_piece_color(1,1).unwrap(), Blue);
    }

    #[test]
    fn move_piece_not_your_piece_error_test() -> Result<(), ()> {
        let mut game_instance = GameInstance::default();

        game_instance
            .put_piece(1, 1, Small)
            .expect("Impossible de placer la pièce");

        match game_instance.move_piece(1, 1, 2, 2) {
            Err(NotYourPiece(_)) => Ok(()),
            _ => Err(()),
        }
    }

    #[test]
    fn move_piece_square_is_empty_test() -> Result<(), ()> {
        let mut game_instance = GameInstance::default();

        match game_instance.move_piece(1, 1, 2, 2) {
            Err(SquareIsEmpty(game_error)) => Ok(()),
            _ => Err(()),
        }
    }

    #[test]
    fn remove_piece_error_test() -> Result<(), ()>{
        let mut game_instance = GameInstance::default();

        game_instance
            .put_piece(0, 0, Small)
            .expect("Impossible de placer la pièce");

        game_instance
            .put_piece(0, 0, Medium)
            .expect("Impossible de placer la pièce");

        game_instance
            .put_piece(1, 1, Small)
            .expect("Impossible de placer la pièce");

        game_instance
            .put_piece(1, 1, Medium)
            .expect("Impossible de placer la pièce");

        match game_instance.put_piece(2, 2, Small) {
            Err(PieceNotAvailable(game_error)) => Ok(()),
            _ => Err(()),
        }
    }

    #[test]
    pub fn game_instance_to_game_state() {
        let mut game_instance = GameInstance::default();

        game_instance.next_turn();

        let game_state = game_instance.to_game_state();

        assert_eq!(game_state.turn, 1);
    }
}

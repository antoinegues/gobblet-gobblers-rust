use crate::model::game_error::GameError;
use crate::model::game_error::GameError::PieceNotAvailable;
use crate::model::game_state::PlayerState;
use crate::model::piece_size::PieceSize::{Big, Medium, Small};
use crate::model::piece::Piece;
use crate::model::piece_size::PieceSize;

pub struct Player {
    pub color: Color,
    pieces: Vec<Piece>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Red,
    Blue,
}

impl Player {
    pub fn new(color: Color) -> Player {
        Player {
            color,
            pieces: vec![
                Piece::new(Small, color),
                Piece::new(Small, color),
                Piece::new(Medium, color),
                Piece::new(Medium, color),
                Piece::new(Big, color),
                Piece::new(Big, color),
            ],
        }
    }

    pub fn remove_piece(&mut self, piece_size: PieceSize) -> Result<Piece, GameError> {
        for i in 0..self.pieces.len() {
            if let Some(piece) = self.pieces.get(i) {
                if piece.size == piece_size {
                    return Ok(self.pieces.remove(i));
                }
            }
        }

        Err(PieceNotAvailable(String::from(
            "Le joueur ne possède plus de pièce de cette taille",
        )))
    }

    pub fn to_player_state(&self) -> PlayerState {
        return PlayerState {
            color: self.color,
            pieces: self.pieces.iter().map(|piece| { piece.size }).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::game_error::GameError::PieceNotAvailable;
    use crate::model::piece_size::PieceSize::{Big, Medium, Small};
    use crate::model::player::Color::Red;
    use crate::model::player::Player;

    #[test]
    fn init_player_test() {
        let player = Player::new(Red);

        assert_eq!(player.color, Red);
        assert_eq!(player.pieces.len(), 6);
        for i in 0..6 {
            let piece = &player.pieces[i];
            if i < 2 {
                assert_eq!(piece.size, Small);
            } else if i < 4 {
                assert_eq!(piece.size, Medium);
            } else {
                assert_eq!(piece.size, Big);
            }

            assert_eq!(piece.color, Red);
        }
    }

    #[test]
    fn remove_piece_test() {
        let mut player = Player::new(Red);

        let mut piece = player.remove_piece(Small).unwrap();

        assert_eq!(piece.size, Small);

        piece = player.remove_piece(Medium).unwrap();

        assert_eq!(piece.size, Medium);

        piece = player.remove_piece(Big).unwrap();

        assert_eq!(piece.size, Big);
        assert_eq!(player.pieces.len(), 3);
    }

    #[test]
    fn remove_piece_error_test() -> Result<(), ()> {
        let mut player = Player::new(Red);

        player.remove_piece(Small).unwrap();
        player.remove_piece(Small).unwrap();

        match player.remove_piece(Small) {
            Err(PieceNotAvailable(_)) => Ok(()),
            _ => Err(()),
        }
    }

    #[test]
    fn player_to_player_state_test() {
        let mut player = Player::new(Red);

        player.remove_piece(Small).unwrap();
        player.remove_piece(Medium).unwrap();
        player.remove_piece(Big).unwrap();
        player.remove_piece(Big).unwrap();

        let player_state = player.to_player_state();

        assert_eq!(player_state.color, Red);

        let mut piece_small_size_count = 0;
        let mut piece_medium_size_count = 0;
        let mut piece_big_size_count = 0;

        for piece_size in player_state.pieces {
            match piece_size {
                Small => piece_small_size_count += 1,
                Medium => piece_medium_size_count += 1,
                Big => piece_big_size_count += 1,
            }
        }

        assert_eq!(piece_small_size_count, 1);
        assert_eq!(piece_medium_size_count, 1);
        assert_eq!(piece_big_size_count, 0);
    }
}

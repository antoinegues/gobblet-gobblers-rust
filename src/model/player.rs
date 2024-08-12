use crate::model::game_error::GameError;
use crate::model::game_error::GameError::PieceNotAvailable;
use crate::model::piece::PieceSize::{Big, Medium, Small};
use crate::model::piece::{Piece, PieceSize};

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
}

#[cfg(test)]
mod tests {
    use crate::model::game_error::GameError::PieceNotAvailable;
    use crate::model::piece::PieceSize::{Big, Medium, Small};
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
}

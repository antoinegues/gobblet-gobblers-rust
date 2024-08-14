use crate::model::game_state::PieceState;
use crate::model::piece_size::PieceSize;
use crate::model::player::Color;

#[derive(Debug)]
pub struct Piece {
    pub size: PieceSize,
    pub color: Color,
    pub nested_piece: Option<Box<Piece>>,
}

impl Piece {
    pub fn new(piece_size: PieceSize, color: Color) -> Piece {
        return Piece {
            size: piece_size,
            color,
            nested_piece: None,
        };
    }

    pub fn remove_nested_piece(&mut self) -> Option<Box<Piece>> {
        self.nested_piece.take()
    }

    pub fn can_be_nested(&self, piece_size: PieceSize) -> bool {
        self.size < piece_size
    }

    pub fn cannot_be_nested(&self, piece_size: PieceSize) -> bool {
        !self.can_be_nested(piece_size)
    }
    pub fn set_nested_piece(&mut self, nested_piece: Piece) {
        self.nested_piece = Some(Box::from(nested_piece));
    }

    pub fn to_piece_state(&self) -> PieceState {
        PieceState {
            color: self.color,
            size: self.size,
            nested_piece: self
                .nested_piece
                .as_ref()
                .map(|nested_piece| Box::from(nested_piece.to_piece_state())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::piece::Piece;
    use crate::model::piece_size::PieceSize::{Big, Medium, Small};
    use crate::model::player::Color::{Blue, Red};

    #[test]
    fn piece_new_test() {
        let piece = Piece::new(Big, Red);

        assert_eq!(piece.size, Big);
        assert_eq!(piece.color, Red);
        assert!(piece.nested_piece.is_none());
    }

    #[test]
    fn piece_can_be_nested_test() {
        let piece = Piece::new(Medium, Red);

        assert!(piece.can_be_nested(Big));
    }

    #[test]
    fn piece_cannot_be_nested_test() {
        let piece = Piece::new(Medium, Red);

        assert!(piece.cannot_be_nested(Small));
    }

    #[test]
    fn set_nested_piece_test() -> Result<(), ()> {
        let mut piece = Piece::new(Medium, Red);
        let nested_piece = Piece::new(Small, Red);

        piece.set_nested_piece(nested_piece);

        match &piece.nested_piece {
            None => Err(()),
            Some(nested_piece) => {
                assert_eq!(nested_piece.size, Small);
                assert_eq!(nested_piece.color, Red);
                Ok(())
            }
        }
    }

    #[test]
    fn remove_nested_piece_test() {
        let mut piece = Piece::new(Medium, Red);

        assert!(piece.remove_nested_piece().is_none());

        piece.set_nested_piece(Piece::new(Small, Blue));

        let old_nested_piece = piece.remove_nested_piece();
        assert!(piece.nested_piece.is_none());
        assert!(old_nested_piece.is_some());

        let old_nested_piece = old_nested_piece.unwrap();
        assert_eq!(old_nested_piece.size, Small);
        assert_eq!(old_nested_piece.color, Blue);
    }

    #[test]
    fn piece_to_piece_state_test() -> Result<(), ()> {
        let mut piece = Piece::new(Big, Red);
        piece.set_nested_piece(Piece::new(Small, Blue));

        let piece_state = piece.to_piece_state();

        assert_eq!(piece_state.color, Red);
        assert_eq!(piece_state.size, Big);
        match piece_state.nested_piece {
            Some(nested_piece) => {
                if nested_piece.color != Blue || nested_piece.size != Small {
                    Err(())
                } else {
                    Ok(())
                }
            }
            _ => Err(()),
        }
    }
}

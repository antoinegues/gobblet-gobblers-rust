use super::piece::{Piece, PieceSize};
use crate::model::game_error::GameError;
use crate::model::game_error::GameError::{CannotPutPieceHere, SquareIsEmpty};
use crate::model::piece::PieceSize::*;
use crate::model::player::Color;


pub struct Board {
    squares: [[Option<Piece>; 3]; 3],
}

impl Default for Board {
    fn default() -> Board {
        Board {
            squares: [[None, None, None], [None, None, None], [None, None, None]],
        }
    }
}

impl Board {
    pub fn square_is_empty(&self, x: usize, y: usize) -> bool {
        match self.squares[x][y] {
            None => true,
            _ => false,
        }
    }

    pub fn square_is_not_empty(&self, x: usize, y: usize) -> bool {
        !self.square_is_empty(x, y)
    }

    pub fn put_piece(&mut self, x: usize, y: usize, mut piece: Piece) -> Result<(), GameError> {
        if let Some(current_piece) = self.squares[x][y].take() {
            if current_piece.cannot_be_nested(&piece) {
                self.squares[x][y] = Some(current_piece);
                return Err(CannotPutPieceHere(String::from(
                    "La pièce est trop petite pour être placer ici",
                )));
            }

            piece.set_nested_piece(current_piece);
        }

        self.squares[x][y] = Some(piece);
        Ok(())
    }

    pub fn get_piece_size(&self, x: usize, y: usize) -> Option<PieceSize> {
        match &self.squares[x][y] {
            None => None,
            Some(piece) => Some((*piece).size.clone()),
        }
    }

    pub fn get_piece_color(&self, x: usize, y: usize) -> Option<Color> {
        match &self.squares[x][y] {
            None => None,
            Some(piece) => Some((*piece).color.clone()),
        }
    }

    fn remove_piece(&mut self, x: usize, y: usize) -> Result<Piece, GameError> {
        let mut piece = match self.squares[x][y].take() {
            None => return Err(SquareIsEmpty(String::from("Cette case est vide"))),
            Some(piece) => piece,
        };

        let new_piece = match piece.remove_nested_piece() {
            None => None,
            Some(nested_piece) => Some(*nested_piece),
        };

        self.squares[x][y] = new_piece;
        Ok(piece)
    }

    pub fn move_piece(
        &mut self,
        origin_x: usize,
        origin_y: usize,
        destination_x: usize,
        destination_y: usize,
    ) -> Result<(), GameError> {
        let piece = self.remove_piece(origin_x, origin_y)?;
        self.put_piece(destination_x, destination_y, piece)
    }

    fn check_line_equals(
        &self,
        x: usize,
        y: usize,
        x_add: isize,
        y_add: isize,
        before_piece: &Piece,
    ) -> bool {
        let new_piece_x = ((x as isize) + x_add) as usize;
        let new_piece_y = ((y as isize) + y_add) as usize;

        if new_piece_x == usize::MAX
            || new_piece_y == usize::MAX
            || new_piece_x > 2
            || new_piece_y > 2
        {
            return true;
        }

        match &self.squares[new_piece_x][new_piece_y] {
            None => false,
            Some(new_piece) => {
                return before_piece.color == new_piece.color
                    && self.check_line_equals(new_piece_x, new_piece_y, x_add, y_add, new_piece)
            }
        }
    }

    fn check_row_win(&self) -> Option<Color> {
        for x in 0..3 {
            match &self.squares[x][0] {
                None => continue,
                Some(piece) => match self.check_line_equals(x, 0, 0, 1, piece) {
                    true => return Some(piece.color.clone()),
                    false => continue,
                },
            }
        }

        None
    }

    fn check_column_win(&self) -> Option<Color> {
        for y in 0..3 {
            match &self.squares[0][y] {
                None => continue,
                Some(piece) => match self.check_line_equals(0, y, 1, 0, piece) {
                    true => return Some(piece.color.clone()),
                    false => continue,
                },
            }
        }
        None
    }

    fn check_diagonal_win(&self) -> Option<Color> {
        let result = match &self.squares[0][0] {
            None => None,
            Some(piece) => match self.check_line_equals(0, 0, 1, 1, piece) {
                true => return Some(piece.color.clone()),
                false => None,
            },
        };

        if result.is_some() {
            return result;
        }

        match &self.squares[0][2] {
            None => None,
            Some(piece) => match self.check_line_equals(0, 2, 1, -1, piece) {
                true => return Some(piece.color.clone()),
                false => None,
            },
        }
    }

    pub fn check_win(&self) -> Option<Color> {
        self.check_column_win()
            .or(self.check_row_win())
            .or(self.check_diagonal_win())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::board::*;
    use crate::model::player::Color::{Blue, Red};

    #[test]
    fn init_board_test() -> Result<(), String> {
        let board = Board::default();

        for row in &board.squares {
            for case in row {
                if let Some(_) = case {
                    return Err(String::from("Le Plateau n'est pas correctement initialisé"));
                }
            }
        }
        Ok(())
    }
    #[test]
    fn put_a_piece_test() {
        let mut board = Board::default();

        assert!(board.square_is_empty(0, 0));

        board
            .put_piece(0, 0, Piece::new(Big, Red))
            .expect("Impossible de place la pièce");

        assert!(board.square_is_not_empty(0, 0));
    }

    #[test]
    fn get_piece_size_test() {
        let mut board = Board::default();

        assert!(board.get_piece_size(0, 0).is_none());

        board
            .put_piece(0, 0, Piece::new(Big, Red))
            .expect("Impossible de place la pièce");

        assert_eq!(board.get_piece_size(0, 0).unwrap(), Big);
    }

    #[test]
    fn get_piece_color_test() {
        let mut board = Board::default();

        assert!(board.get_piece_size(0, 0).is_none());

        board
            .put_piece(0, 0, Piece::new(Big, Red))
            .expect("Impossible de place la pièce");

        assert_eq!(board.get_piece_color(0, 0).unwrap(), Red);
    }

    #[test]
    fn put_a_piece_above_a_piece_test() {
        let mut board = Board::default();

        assert!(board.get_piece_size(0, 0).is_none());

        board
            .put_piece(0, 0, Piece::new(Small, Red))
            .expect("Impossible de placer la pièce");

        assert_eq!(board.get_piece_size(0, 0).unwrap(), Small);

        board
            .put_piece(0, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert_eq!(board.get_piece_size(0, 0).unwrap(), Medium);
    }

    #[test]
    fn get_piece_size_with_nested_test() {
        let mut board = Board::default();

        assert!(board.get_piece_size(0, 0).is_none());

        board
            .put_piece(0, 0, Piece::new(Small, Red))
            .expect("Impossible de place la pièce");

        assert_eq!(board.get_piece_size(0, 0).unwrap(), Small);

        board
            .put_piece(0, 0, Piece::new(Medium, Red))
            .expect("Impossible de place la pièce");

        assert_eq!(board.get_piece_size(0, 0).unwrap(), Medium);
    }

    #[test]
    fn get_piece_color_with_nested_test() {
        let mut board = Board::default();

        assert!(board.get_piece_size(0, 0).is_none());

        board
            .put_piece(0, 0, Piece::new(Medium, Red))
            .expect("Impossible de place la pièce");

        assert_eq!(board.get_piece_color(0, 0).unwrap(), Red);

        board
            .put_piece(0, 0, Piece::new(Big, Blue))
            .expect("Impossible de place la pièce");

        assert_eq!(board.get_piece_color(0, 0).unwrap(), Blue);
    }

    #[test]
    fn put_a_piece_above_a_too_big_piece_error_test() {
        let mut board = Board::default();

        assert!(board.square_is_empty(0, 0));

        board
            .put_piece(0, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert_eq!(board.get_piece_size(0, 0).unwrap(), Medium);

        assert!(board.put_piece(0, 0, Piece::new(Small, Red)).is_err());
        assert!(board.square_is_not_empty(0, 0));
    }

    #[test]
    fn remove_a_piece_test() {
        let mut board = Board::default();

        board
            .put_piece(0, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        board
            .remove_piece(0, 0)
            .expect("Impossible de retirer la pièce");

        assert!(board.square_is_empty(0, 0));
    }

    #[test]
    fn remove_a_piece_on_empty_square_error_test() -> Result<(), ()> {
        let mut board = Board::default();

        match board.remove_piece(0, 0) {
            Err(SquareIsEmpty(_)) => Ok(()),
            _ => Err(()),
        }
    }

    #[test]
    fn remove_a_piece_with_nested_piece_test() {
        let mut board = Board::default();

        board
            .put_piece(0, 0, Piece::new(Small, Red))
            .expect("Impossible de placer la pièce");
        board
            .put_piece(0, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");
        board
            .remove_piece(0, 0)
            .expect("Impossible de retirer la pièce");

        assert!(board.square_is_not_empty(0, 0));
        assert_eq!(board.get_piece_size(0, 0).unwrap(), Small);
    }

    #[test]
    fn move_a_piece_to_a_empty_square_test() {
        let mut board = Board::default();

        board
            .put_piece(0, 0, Piece::new(Small, Red))
            .expect("Impossible de placer la pièce");

        board
            .move_piece(0, 0, 1, 1)
            .expect("Impossible de déplacer la pièce");

        assert!(board.square_is_empty(0, 0));
        assert!(board.square_is_not_empty(1, 1));
        assert_eq!(board.get_piece_size(1, 1).unwrap(), Small);
    }

    #[test]
    fn move_a_piece_from_empty_square_error_test() -> Result<(), ()> {
        let mut board = Board::default();

        match board.move_piece(0, 0, 1, 1) {
            Err(SquareIsEmpty(_)) => Ok(()),
            _ => Err(()),
        }
    }

    #[test]
    fn move_a_piece_on_a_too_big_piece_error_test() -> Result<(), ()> {
        let mut board = Board::default();

        board
            .put_piece(0, 0, Piece::new(Small, Red))
            .expect("Impossible de placer la pièce");
        board
            .put_piece(1, 1, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        match board.move_piece(0, 0, 1, 1) {
            Err(CannotPutPieceHere(_)) => Ok(()),
            _ => Err(()),
        }
    }

    #[test]
    fn move_a_piece_on_a_piece_test() {
        let mut board = Board::default();

        board
            .put_piece(0, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");
        board
            .put_piece(1, 1, Piece::new(Small, Red))
            .expect("Impossible de placer la pièce");
        board
            .move_piece(0, 0, 1, 1)
            .expect("Impossible de retirer la pièce");

        assert_eq!(board.get_piece_size(1, 1).unwrap(), Medium);
    }

    #[test]
    fn check_row_win_test() {
        let mut board = Board::default();

        assert!(board.check_win().is_none());

        board
            .put_piece(1, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");
        board
            .put_piece(1, 1, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());

        board
            .put_piece(1, 2, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert_eq!(board.check_win().unwrap(), Red);
    }

    #[test]
    fn check_row_no_win_test() {
        let mut board = Board::default();

        assert!(board.check_win().is_none());

        board
            .put_piece(1, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");
        board
            .put_piece(1, 1, Piece::new(Medium, Blue))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());

        board
            .put_piece(1, 2, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());
    }

    #[test]
    fn check_column_win_test() {
        let mut board = Board::default();

        assert!(board.check_win().is_none());

        board
            .put_piece(0, 2, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");
        board
            .put_piece(1, 2, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());

        board
            .put_piece(2, 2, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert_eq!(board.check_win().unwrap(), Red);
    }

    #[test]
    fn check_column_no_win_test() {
        let mut board = Board::default();

        assert!(board.check_win().is_none());

        board
            .put_piece(0, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");
        board
            .put_piece(1, 0, Piece::new(Medium, Blue))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());

        board
            .put_piece(2, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());
    }

    #[test]
    fn check_diagonal_win_test() {
        let mut board = Board::default();

        assert!(board.check_win().is_none());

        board
            .put_piece(0, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");
        board
            .put_piece(1, 1, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());

        board
            .put_piece(2, 2, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert_eq!(board.check_win().unwrap(), Red);

        let mut board = Board::default();

        assert!(board.check_win().is_none());

        board
            .put_piece(0, 2, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");
        board
            .put_piece(1, 1, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());

        board
            .put_piece(2, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert_eq!(board.check_win().unwrap(), Red);
    }

    #[test]
    fn check_diagonal_no_win_test() {
        let mut board = Board::default();

        assert!(board.check_win().is_none());

        board
            .put_piece(0, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");
        board
            .put_piece(1, 1, Piece::new(Medium, Blue))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());

        board
            .put_piece(2, 2, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());

        let mut board = Board::default();

        assert!(board.check_win().is_none());

        board
            .put_piece(0, 2, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");
        board
            .put_piece(1, 1, Piece::new(Medium, Blue))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());

        board
            .put_piece(2, 0, Piece::new(Medium, Red))
            .expect("Impossible de placer la pièce");

        assert!(board.check_win().is_none());
    }
}

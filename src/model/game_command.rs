use crate::model::piece_size::PieceSize;

pub enum GameCommand {
    NewGameCommand,
    PutPieceCommand(usize, usize, PieceSize),
    MovePieceCommand(usize, usize, usize, usize),
}

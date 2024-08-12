#[derive(Debug, Clone)]
pub enum GameError {
    CurrentlyNoGame(String),
    CannotPutPieceHere(String),
    SquareIsEmpty(String),
    PieceNotAvailable(String),
    NotYourPiece(String),
    UnknownError,
}

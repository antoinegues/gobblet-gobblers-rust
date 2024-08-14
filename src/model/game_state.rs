use crate::model::piece_size::PieceSize;
use crate::model::player::Color;

#[derive(Clone, Debug)]
pub struct GameState {
    pub board: BoardState,
    pub players: [PlayerState; 2],
    pub turn: u32,
    pub winner_color: Option<Color>,
}

#[derive(Clone, Debug)]
pub struct BoardState {
    pub squares: [[Option<PieceState>; 3]; 3],
}

#[derive(Clone, Debug)]
pub struct PieceState {
    pub color: Color,
    pub size: PieceSize,
    pub nested_piece: Option<Box<PieceState>>,
}

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub color: Color,
    pub pieces: Vec<PieceSize>,
}

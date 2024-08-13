use crate::model::piece::{PieceSize};
use crate::model::player::Color;

#[derive(Clone)]
pub struct GameState {
    pub board: BoardState,
    pub players: [PlayerState; 2],
    pub turn: u32,
}

#[derive(Clone)]
pub struct BoardState {
    pub squares: [[Option<PieceState>; 3]; 3],
}

#[derive(Clone)]
pub struct PieceState {
    pub color: Color,
    pub size: PieceSize,
    pub nested_piece: Option<Box<PieceState>>
}

#[derive(Clone)]
pub struct PlayerState {
    pub color: Color,
    pub pieces: Vec<PieceSize>,
}
use crate::pieces::PieceColor;
use crate::position::Position;
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Normal,
    Testing,
}

pub struct Game {
    pub position: Position,
    pub turn: PieceColor,
    pub mode: GameMode,
}

impl Game {
    pub fn new() -> Self {
        Self {
            position: Position::new(),
            turn: PieceColor::White,
            mode: GameMode::Testing,
        }
    }
    pub fn can_select(&self, color: PieceColor) -> bool {
        match self.mode {
            GameMode::Testing => true,
            GameMode::Normal => color == self.turn,
        }
    }
    pub fn should_switch_turns(&self) -> bool {
        matches!(self.mode, GameMode::Normal)
    }
    pub fn next_turn(&mut self) {
        self.turn = match self.turn {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
    }
    pub fn move_piece(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
        self.position.move_piece(from_row, from_col, to_row, to_col);

        if self.should_switch_turns() {
            self.next_turn();
        }
    }
}

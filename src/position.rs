use raylib::prelude::*;

use crate::pieces::{Piece, PieceColor, PieceType, draw_piece};

pub struct Position {
    pub board: [[Option<Piece>; 8]; 8], //In the board where we are saving the posicition, there MIGHT be a Piece. In the 8x8 matrix.
}

impl Position {
    pub fn new() -> Self {
        let mut board = [[None; 8]; 8];

        let back_rank = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        // For non peons
        for col in 0..8 {
            board[0][col] = Some(Piece {
                piece_type: back_rank[col],
                piece_color: PieceColor::Black,
            });

            board[7][col] = Some(Piece {
                piece_type: back_rank[col],
                piece_color: PieceColor::White,
            });
        }

        // Peons
        for col in 0..8 {
            board[1][col] = Some(Piece {
                piece_type: PieceType::Pawn,
                piece_color: PieceColor::Black,
            });

            board[6][col] = Some(Piece {
                piece_type: PieceType::Pawn,
                piece_color: PieceColor::White,
            });
        }

        Self { board }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = &self.board[row][col] {
                draw_piece(piece, row as i32, col as i32, d);
            }
        }
    }
}
}


use raylib::prelude::*;

use crate::{
    pieces::{Piece, PieceColor, PieceType, draw_piece},
    textures::PieceTextures,
};

pub struct Position {
    pub board: [[Option<Piece>; 8]; 8], //In the board where we are saving the posicition, there MIGHT be a Piece. In the 8x8 matrix.

    pub white_king_moved: bool, ////////////// //////////////
    pub black_king_moved: bool, ////////////// //////////////

    pub white_left_rook_moved: bool,  //Used for castling mechanic
    pub white_right_rook_moved: bool, ////////////// //////////////

    pub black_left_rook_moved: bool,  ////////////// //////////////
    pub black_right_rook_moved: bool, ////////////// //////////////

    pub en_passant_square: Option<(usize, usize)>, //Assings the tile of a possible en passant capture.

    pub promotion: Option<(usize, usize)>, //Checks coordinates (x, y) for a pawn that might be promoted.
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

        Self {
            board,
            white_king_moved: false,
            black_king_moved: false,
            white_left_rook_moved: false,
            white_right_rook_moved: false,
            black_left_rook_moved: false,
            black_right_rook_moved: false,
            en_passant_square: None,
            promotion: None,
        }
    }

    pub fn draw(
        &self,
        d: &mut RaylibDrawHandle,
        dragging_square: Option<(usize, usize)>,
        textures: &PieceTextures,
    ) {
        for row in 0..8 {
            for col in 0..8 {
                // If it's the dragging piece, do not draw
                if dragging_square == Some((row, col)) {
                    continue;
                }

                if let Some(piece) = &self.board[row][col] {
                    draw_piece(piece, row as i32, col as i32, textures, d);
                }
            }
        }
    }

    pub fn move_piece(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
        let piece = self.board[from_row][from_col];

        if let Some(piece) = piece {
            // En passant capture
            if piece.piece_type == PieceType::Pawn {
                if let Some((ep_row, ep_col)) = self.en_passant_square {
                    if to_row == ep_row && to_col == ep_col {
                        match piece.piece_color {
                            PieceColor::White => {
                                // Remove the black pawn behind the destination square
                                self.board[to_row + 1][to_col] = None;
                            }

                            PieceColor::Black => {
                                // Remove the white pawn behind the destination square
                                self.board[to_row - 1][to_col] = None;
                            }
                        }
                    }
                }
            }

            match (piece.piece_color, piece.piece_type) {
                (PieceColor::White, PieceType::King) => {
                    self.white_king_moved = true;

                    // Short castle
                    if from_row == 7 && from_col == 4 && to_row == 7 && to_col == 6 {
                        let rook = self.board[7][7];
                        self.board[7][7] = None;
                        self.board[7][5] = rook;
                    }

                    // Long castle
                    if from_row == 7 && from_col == 4 && to_row == 7 && to_col == 2 {
                        let rook = self.board[7][0];
                        self.board[7][0] = None;
                        self.board[7][3] = rook;
                    }
                }

                (PieceColor::Black, PieceType::King) => {
                    self.black_king_moved = true;

                    // Short castle
                    if from_row == 0 && from_col == 4 && to_row == 0 && to_col == 6 {
                        let rook = self.board[0][7];
                        self.board[0][7] = None;
                        self.board[0][5] = rook;
                    }

                    // Long castle
                    if from_row == 0 && from_col == 4 && to_row == 0 && to_col == 2 {
                        let rook = self.board[0][0];
                        self.board[0][0] = None;
                        self.board[0][3] = rook;
                    }
                }

                (PieceColor::White, PieceType::Rook) => {
                    if from_row == 7 && from_col == 0 {
                        self.white_left_rook_moved = true; // all this to change the flag to true if either king or rook moves.
                    }

                    if from_row == 7 && from_col == 7 {
                        self.white_right_rook_moved = true;
                    }
                }

                (PieceColor::Black, PieceType::Rook) => {
                    if from_row == 0 && from_col == 0 {
                        self.black_left_rook_moved = true;
                    }

                    if from_row == 0 && from_col == 7 {
                        self.black_right_rook_moved = true;
                    }
                }

                _ => {}
            }
        }

        self.board[from_row][from_col] = None;
        self.board[to_row][to_col] = piece;

        // Promotion logic
        if let Some(piece) = piece {
            if piece.piece_type == PieceType::Pawn {
                match piece.piece_color {
                    PieceColor::White => {
                        if to_row == 0 {
                            self.promotion = Some((to_row, to_col));
                        }
                    }

                    PieceColor::Black => {
                        if to_row == 7 {
                            self.promotion = Some((to_row, to_col));
                        }
                    }
                }
            }
        }

        // En passant is only available for the next move.
        self.en_passant_square = None;

        if let Some(piece) = piece {
            if piece.piece_type == PieceType::Pawn {
                // White pawn double move
                if from_row == 6 && to_row == 4 {
                    self.en_passant_square = Some((5, from_col));
                }

                // Black pawn double move
                if from_row == 1 && to_row == 3 {
                    self.en_passant_square = Some((2, from_col));
                }
            }
        }
    }

    pub fn promote(&mut self, piece_type: PieceType) {
        if let Some((row, col)) = self.promotion {
            let pawn = self.board[row][col].unwrap();

            self.board[row][col] = Some(Piece {
                piece_type,
                piece_color: pawn.piece_color,
            });

            self.promotion = None;
        }
    }
}

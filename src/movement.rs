
use crate::{
    pieces::{Piece, PieceColor, PieceType},
    position::Position,
};

/*
==================
LEGAL MOVES
==================
*/

const ROOK_DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

const BISHOP_DIRECTIONS: [(i32, i32); 4] = [
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

const QUEEN_DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

const KING_DIRECTIONS: [(i32, i32); 8] = QUEEN_DIRECTIONS;

const KNIGHT_OFFSETS: [(i32, i32); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

/*
==================
EXISTING TILE
==================
*/

fn inside_board(row: i32, col: i32) -> bool {
    row >= 0 && row < 8 && col >= 0 && col < 8
}

/*
==================
ENEMY PIECE IN BETWEEN
==================
*/

fn is_enemy(piece: &Piece, other: &Piece) -> bool {
    piece.piece_color != other.piece_color
}

pub fn legal_moves(
    piece: &Piece,
    position: &Position,
    row: usize,
    col: usize,
) -> Vec<(usize, usize)> {
    match piece.piece_type {
        PieceType::Rook => sliding_moves(piece, position, row, col, &ROOK_DIRECTIONS),

        PieceType::Bishop => sliding_moves(piece, position, row, col, &BISHOP_DIRECTIONS),

        PieceType::Queen => sliding_moves(piece, position, row, col, &QUEEN_DIRECTIONS),

        PieceType::King => single_moves(piece, position, row, col, &KING_DIRECTIONS),

        PieceType::Knight => knight_moves(piece, position, row, col),

        PieceType::Pawn => pawn_moves(piece, position, row, col),
    }
}

/*
===================================
SLIDING MOVES (BISHOP, QUEEN, ROOK)
===================================
*/

fn sliding_moves(
    piece: &Piece,
    position: &Position,
    row: usize,
    col: usize,
    directions: &[(i32, i32)],
) -> Vec<(usize, usize)> {

    let mut moves = Vec::new();

    for &(dr, dc) in directions {

        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;

        while inside_board(r, c) {

            match position.board[r as usize][c as usize] {

                None => {
                    moves.push((r as usize, c as usize));

                    r += dr;
                    c += dc;
                }

                Some(other_piece) => {

                    if is_enemy(piece, &other_piece) {
                        moves.push((r as usize, c as usize));
                    }

                    break;
                }
            }
        }
    }

    moves
}

/*
===================================
SINGLE MOVES (KING)
===================================
*/

fn single_moves(
    piece: &Piece,
    position: &Position,
    row: usize,
    col: usize,
    directions: &[(i32, i32)],
) -> Vec<(usize, usize)> {

    let mut moves = Vec::new();

    for &(dr, dc) in directions {

        let r = row as i32 + dr;
        let c = col as i32 + dc;

        if !inside_board(r, c) {
            continue;
        }

        match position.board[r as usize][c as usize] {

            None => {
                moves.push((r as usize, c as usize));
            }

            Some(other_piece) => {

                if is_enemy(piece, &other_piece) {
                    moves.push((r as usize, c as usize));
                }

            }

        }

    }

    moves
}

/*
===================================
KNIGHT MOVES
===================================
*/

fn knight_moves(
    piece: &Piece,
    position: &Position,
    row: usize,
    col: usize,
) -> Vec<(usize, usize)> {

    let mut moves = Vec::new();

    for &(dr, dc) in &KNIGHT_OFFSETS {

        let r = row as i32 + dr;
        let c = col as i32 + dc;

        if !inside_board(r, c) {
            continue;
        }

        match position.board[r as usize][c as usize] {

            None => {
                moves.push((r as usize, c as usize));
            }

            Some(other_piece) => {
                if is_enemy(piece, &other_piece) {
                    moves.push((r as usize, c as usize));
                }
            }
        }
    }

    moves
}

/*
===================================
PAWN MOVES (En passant, queening, capture)
===================================
*/

fn pawn_moves(
    piece: &Piece,
    position: &Position,
    row: usize,
    col: usize,
) -> Vec<(usize, usize)> {

    let mut moves = Vec::new();

    // White goes up, black goes down.
    let direction = match piece.piece_color {
        PieceColor::White => -1,
        PieceColor::Black => 1,
    };

    let starting_row = match piece.piece_color {
        PieceColor::White => 6,
        PieceColor::Black => 1,
    };

    /*
    ==================
    FORWARD MOVEMENT
    ==================
    */

    let next_row = row as i32 + direction;

    if inside_board(next_row, col as i32)
        && position.board[next_row as usize][col].is_none()
    {
        // One square
        moves.push((next_row as usize, col));

        // Two squares from starting position
        if row == starting_row {

            let double_row = row as i32 + direction * 2;

            if inside_board(double_row, col as i32)
                && position.board[double_row as usize][col].is_none()
            {
                moves.push((double_row as usize, col));
            }
        }
    }

    /*
    ==================
    DIAGONAL CAPTURES
    ==================
    */

    let capture_offsets = match piece.piece_color {
        PieceColor::White => [(-1, -1), (-1, 1)],
        PieceColor::Black => [(1, -1), (1, 1)],
    };

    for &(dr, dc) in &capture_offsets {

        let r = row as i32 + dr;
        let c = col as i32 + dc;

        if !inside_board(r, c) {
            continue;
        }

        if let Some(other_piece) = position.board[r as usize][c as usize] {

            if is_enemy(piece, &other_piece) {
                moves.push((r as usize, c as usize));
            }
        }
    }

    moves
}
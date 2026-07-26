use raylib::prelude::*;

/*
==================
PIECE DEFINITION
==================
*/
#[derive(Copy, Clone)]
pub enum PieceType{
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}
#[derive(Copy, Clone)]
pub enum PieceColor{
    White,
    Black,
}
#[derive(Copy, Clone)]
pub struct Piece{
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
}

/*
==================
PIECE DRAWING
==================
*/

pub fn draw_piece(piece: &Piece, row: i32, column: i32, d: &mut RaylibDrawHandle) {
    let tile_size = 100;

    let screen_x = column * tile_size;
    let screen_y = row * tile_size;

    draw_piece_pixels(piece, screen_x, screen_y, d);
}

pub fn draw_piece_pixels(
    piece: &Piece,
    x: i32,
    y: i32,
    d: &mut RaylibDrawHandle,
) {
    let symbol = match (piece.piece_color, piece.piece_type) {
        (PieceColor::White, PieceType::King) => "WK",
        (PieceColor::White, PieceType::Queen) => "WQ",
        (PieceColor::White, PieceType::Rook) => "WR",
        (PieceColor::White, PieceType::Bishop) => "WB",
        (PieceColor::White, PieceType::Knight) => "WKN",
        (PieceColor::White, PieceType::Pawn) => "WP",

        (PieceColor::Black, PieceType::King) => "BK",
        (PieceColor::Black, PieceType::Queen) => "BQ",
        (PieceColor::Black, PieceType::Rook) => "BR",
        (PieceColor::Black, PieceType::Bishop) => "BB",
        (PieceColor::Black, PieceType::Knight) => "BKN",
        (PieceColor::Black, PieceType::Pawn) => "BP",
    };

    d.draw_text(symbol, x, y, 32, Color::RED);
}
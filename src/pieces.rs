use raylib::prelude::*;

use crate::textures::PieceTextures;

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

pub fn draw_piece(piece: &Piece, row: i32, column: i32, textures: &PieceTextures, d: &mut RaylibDrawHandle) {
    let tile_size = 100;

    let screen_x = column * tile_size;
    let screen_y = row * tile_size;

    draw_piece_pixels(piece, screen_x, screen_y, d, textures);
}

pub fn draw_piece_pixels(
    piece: &Piece,
    x: i32,
    y: i32,
    d: &mut RaylibDrawHandle,
    textures: &PieceTextures,
) {
    let texture = match (piece.piece_color, piece.piece_type) {
    (PieceColor::White, PieceType::King) => &textures.w_king,
    (PieceColor::White, PieceType::Queen) => &textures.w_queen,
    (PieceColor::White, PieceType::Rook) => &textures.w_rook,
    (PieceColor::White, PieceType::Bishop) => &textures.w_bishop,
    (PieceColor::White, PieceType::Knight) => &textures.w_knight,
    (PieceColor::White, PieceType::Pawn) => &textures.w_pawn,

    (PieceColor::Black, PieceType::King) => &textures.b_king,
    (PieceColor::Black, PieceType::Queen) => &textures.b_queen,
    (PieceColor::Black, PieceType::Rook) => &textures.b_rook,
    (PieceColor::Black, PieceType::Bishop) => &textures.b_bishop,
    (PieceColor::Black, PieceType::Knight) => &textures.b_knight,
    (PieceColor::Black, PieceType::Pawn) => &textures.b_pawn,
    
    }; 

    let source = Rectangle {
    x: 0.0,
    y: 0.0,
    width: texture.width as f32,
    height: texture.height as f32,
    };

    let desired_height = 90.0;

    let scale = desired_height / texture.height as f32;

    let width = texture.width as f32 * scale;
    let height = texture.height as f32 * scale;

    let dest = Rectangle {
    x: x as f32 + (100.0 - width) / 2.0,
    y: y as f32 + (100.0 - height) / 2.0,
    width,
    height,
    };

    d.draw_texture_pro(
        texture,
        source,
        dest,
        Vector2::zero(),
        0.0,
        Color::WHITE,
    );
}
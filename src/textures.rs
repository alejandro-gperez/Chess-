use raylib::prelude::*;

pub struct PieceTextures {
    pub w_king: Texture2D,
    pub w_queen: Texture2D,
    pub w_rook: Texture2D,
    pub w_bishop: Texture2D,
    pub w_knight: Texture2D,
    pub w_pawn: Texture2D,

    pub b_king: Texture2D,
    pub b_queen: Texture2D,
    pub b_rook: Texture2D,
    pub b_bishop: Texture2D,
    pub b_knight: Texture2D,
    pub b_pawn: Texture2D,
}

impl PieceTextures {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Self {

        Self {
            w_king: rl.load_texture(thread, "assets/W_King.png").unwrap(),
            w_queen: rl.load_texture(thread, "assets/W_Queen.png").unwrap(),
            w_rook: rl.load_texture(thread, "assets/W_Rook.png").unwrap(),
            w_bishop: rl.load_texture(thread, "assets/W_Bishop.png").unwrap(),
            w_knight: rl.load_texture(thread, "assets/W_Knight.png").unwrap(),
            w_pawn: rl.load_texture(thread, "assets/W_Pawn.png").unwrap(),

            b_king: rl.load_texture(thread, "assets/B_King.png").unwrap(),
            b_queen: rl.load_texture(thread, "assets/B_Queen.png").unwrap(),
            b_rook: rl.load_texture(thread, "assets/B_Rook.png").unwrap(),
            b_bishop: rl.load_texture(thread, "assets/B_Bishop.png").unwrap(),
            b_knight: rl.load_texture(thread, "assets/B_Knight.png").unwrap(),
            b_pawn: rl.load_texture(thread, "assets/B_Pawn.png").unwrap(),
        }
    }
}
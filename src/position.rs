use crate::{
    fen::{parse_fen, BoardState, Piece},
    util::{mailbox_to_bb, Bitboard, Color},
};

pub struct Position {
    board_state: BoardState,
    pub pieces: [Bitboard; 12],
}

impl From<&str> for Position {
    fn from(fen: &str) -> Self {
        let parsed_fen = parse_fen(fen).unwrap();
        let mut pieces = [0u64; 12];

        for (idx, piece_char) in Piece::PIECE_CHARS.iter().enumerate() {
            let bb = mailbox_to_bb(parsed_fen.0, Piece::from_char(*piece_char).unwrap());
            pieces[idx] = bb;
        }

        Self {
            pieces,
            board_state: parsed_fen,
        }
    }
}

impl Position {
    pub fn generate_pawn_moves() {}

    pub fn all_pieces_bb(&self, color: Option<Color>) -> Bitboard {
        match color {
            Some(Color::White) => {
                let mut bb = 0u64;
                for idx in 0usize..6 {
                    bb |= self.pieces[idx]
                }
                bb
            }
            Some(Color::Black) => {
                let mut bb = 0u64;
                for idx in 6usize..12 {
                    bb |= self.pieces[idx]
                }
                bb
            }
            _ => {
                Self::all_pieces_bb(&self, Some(Color::White))
                    | Self::all_pieces_bb(&self, Some(Color::Black))
            }
        }
    }
}

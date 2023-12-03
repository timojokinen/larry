use crate::{
    attacks::{
        lookup_bishop_att, lookup_knight_att, lookup_pawn_att, lookup_queen_att, lookup_rook_att,
    },
    cmove::{Move, MoveFlags},
    fen::{parse_fen, BoardState},
    piece::{Piece, PieceTypes},
    util::{bb_from_square, enumerate_bits, mailbox_to_bb, opp, relative_rank, Bitboard, Color},
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
    pub fn generate_moves(&self) {
        let ally_color = self.board_state.1;
        let opp_color = opp(ally_color);

        let ally_pieces_bb = self.all_pieces_bb(ally_color.into());
        let opp_pieces_bb = self.all_pieces_bb(opp_color.into());
        let all_pieces_bb = ally_pieces_bb | opp_pieces_bb;
        let mut moves: Vec<Move> = Vec::new();

        let knight = Piece::new(ally_color, PieceTypes::KNIGHT);

        // Knight moves
        // TODO: Illegal moves
        enumerate_bits(self.pieces[knight.get_index()], |from_sq| {
            let att = lookup_knight_att(from_sq) & !all_pieces_bb;
            let captures = att & opp_pieces_bb;
            let quiet = att & !opp_pieces_bb;

            enumerate_bits(captures, |to_sq| {
                moves.push(Move::new(from_sq.into(), to_sq.into(), MoveFlags::CAPTURE));
            });
            enumerate_bits(quiet, |to_sq| {
                moves.push(Move::new(from_sq.into(), to_sq.into(), MoveFlags::QUIET));
            });
        });

        // Pawn moves
        // TODO: illegal moves
        let pawn = Piece::new(ally_color, PieceTypes::PAWN);
        let pawn_direction: i8 = if ally_color == Color::White { 1 } else { -1 };
        let prom_rank = relative_rank(7, ally_color);

        enumerate_bits(self.pieces[pawn.get_index()], |from_sq| {
            let to_sq = (from_sq as i8 + 8 * pawn_direction) as u8;
            let to_bb = bb_from_square(to_sq);
            let from_bb = bb_from_square(from_sq);

            if to_bb & !all_pieces_bb != 0 {
                // Pawn promotions
                if to_bb & prom_rank != 0 {
                    moves.push(Move::new(
                        from_sq.into(),
                        to_sq.into(),
                        MoveFlags::KNIGHT_PROM,
                    ));
                    moves.push(Move::new(
                        from_sq.into(),
                        to_sq.into(),
                        MoveFlags::BISHOP_PROM,
                    ));
                    moves.push(Move::new(
                        from_sq.into(),
                        to_sq.into(),
                        MoveFlags::ROOK_PROM,
                    ));
                    moves.push(Move::new(
                        from_sq.into(),
                        to_sq.into(),
                        MoveFlags::QUEEN_PROM,
                    ));
                }
                // Pawn single push
                else {
                    moves.push(Move::new(from_sq.into(), to_sq.into(), MoveFlags::QUIET));
                }

                // Pawn double push
                let to_sq = (to_sq as i8 + 8 * pawn_direction) as u8;
                if bb_from_square(to_sq) & !all_pieces_bb != 0
                    && from_bb & relative_rank(1, ally_color) != 0
                {
                    moves.push(Move::new(
                        from_sq.into(),
                        to_sq.into(),
                        MoveFlags::DOUBLE_PAWN_PUSH,
                    ));
                }
            }

            let att = lookup_pawn_att(from_sq, ally_color) & opp_pieces_bb;
            let captures = att & !prom_rank;
            let prom_captures = att & prom_rank;

            // Capture pawn moves
            enumerate_bits(captures, |to_sq| {
                moves.push(Move::new(from_sq.into(), to_sq.into(), MoveFlags::CAPTURE))
            });

            // Promotion capture pawn moves
            enumerate_bits(prom_captures, |to_sq| {
                moves.push(Move::new(
                    from_sq.into(),
                    to_sq.into(),
                    MoveFlags::BISHOP_PROM_CAPTURE,
                ));
                moves.push(Move::new(
                    from_sq.into(),
                    to_sq.into(),
                    MoveFlags::ROOK_PROM_CAPTURE,
                ));
                moves.push(Move::new(
                    from_sq.into(),
                    to_sq.into(),
                    MoveFlags::QUEEN_PROM_CAPTURE,
                ));
                moves.push(Move::new(
                    from_sq.into(),
                    to_sq.into(),
                    MoveFlags::KNIGHT_PROM_CAPTURE,
                ));
            });
        });

        let queen = Piece::new(ally_color, PieceTypes::QUEEN);

        // Queen moves
        enumerate_bits(self.pieces[queen.get_index()], |from_sq| {
            let att = lookup_queen_att(from_sq, all_pieces_bb) & !ally_pieces_bb;
            let quiet = att & !opp_pieces_bb;
            let captures = att & opp_pieces_bb;

            enumerate_bits(quiet, |to_sq| {
                moves.push(Move::new(from_sq.into(), to_sq.into(), MoveFlags::QUIET));
            });
            enumerate_bits(captures, |to_sq| {
                moves.push(Move::new(from_sq.into(), to_sq.into(), MoveFlags::CAPTURE));
            });
        });

        let bishop = Piece::new(ally_color, PieceTypes::BISHOP);

        // Bishop moves
        enumerate_bits(self.pieces[bishop.get_index()], |from_sq| {
            let att = lookup_bishop_att(from_sq, all_pieces_bb) & !ally_pieces_bb;
            let quiet = att & !opp_pieces_bb;
            let captures = att & opp_pieces_bb;

            enumerate_bits(quiet, |to_sq| {
                moves.push(Move::new(from_sq.into(), to_sq.into(), MoveFlags::QUIET));
            });
            enumerate_bits(captures, |to_sq| {
                moves.push(Move::new(from_sq.into(), to_sq.into(), MoveFlags::CAPTURE));
            });
        });

        let rook = Piece::new(ally_color, PieceTypes::ROOK);

        // Rook moves
        enumerate_bits(self.pieces[rook.get_index()], |from_sq| {
            let att = lookup_rook_att(from_sq, all_pieces_bb) & !ally_pieces_bb;
            let quiet = att & !opp_pieces_bb;
            let captures = att & opp_pieces_bb;

            enumerate_bits(quiet, |to_sq| {
                moves.push(Move::new(from_sq.into(), to_sq.into(), MoveFlags::QUIET));
            });
            enumerate_bits(captures, |to_sq| {
                moves.push(Move::new(from_sq.into(), to_sq.into(), MoveFlags::CAPTURE));
            });
        });
    }

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

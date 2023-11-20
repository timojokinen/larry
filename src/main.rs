mod attacks;
mod castling_rights;
mod fen;
mod util;

use attacks::{attacks::pawn_att, init_tables};
use util::*;

fn main() {
    /* let res = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let BoardState(mailbox, _, _, _, _, _) = res;
    let mut bbs: [Bitboard; 12] = [0; 12];
    for (idx, piece) in Piece::PIECE_CHARS.iter().enumerate() {
        bbs[idx] = mailbox_to_bitboard(mailbox, *piece);
    } */

    init_tables();

    print_bitboard(pawn_att(8, Color::Black));
}

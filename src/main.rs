mod attacks;
mod castling_rights;
mod fen;
mod position;
mod util;

use attacks::init_tables;
use position::Position;
use util::*;

fn main() {
    init_tables();

    let position = Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let pieces = position.pieces[1];
    print_bitboard(pieces);
    iterate_bits(pieces, |idx| {
        println!("{}", idx);
    })
}

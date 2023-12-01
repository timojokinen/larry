mod attacks;
mod castling_rights;
mod cmove;
mod fen;
mod piece;
mod position;
mod util;

use attacks::init_tables;
use position::Position;
use util::*;

fn main() {
    init_tables();

    let position = Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    position.generate_moves();
}

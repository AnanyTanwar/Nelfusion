use crate::board::position::{PieceType, Position};
use crate::movegen::movegen::generate_pseudo_legal;
use crate::movegen::moves::MoveList;

pub fn perft(pos: &mut Position, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut list = MoveList::new();
    generate_pseudo_legal(pos, &mut list);

    let mover = pos.side_to_move;
    let mut nodes = 0u64;

    for &m in list.iter() {
        let info = pos.make_move(m);

        if !own_king_in_check(pos, mover) {
            nodes += perft(pos, depth - 1);
        }

        pos.unmake_move(m, info);
    }

    nodes
}

pub fn perft_divide(pos: &mut Position, depth: u32) -> u64 {
    let mut list = MoveList::new();
    generate_pseudo_legal(pos, &mut list);

    let mover = pos.side_to_move;
    let mut total = 0u64;

    for &m in list.iter() {
        let info = pos.make_move(m);

        if !own_king_in_check(pos, mover) {
            let nodes = if depth == 0 { 1 } else { perft(pos, depth - 1) };
            println!("{}: {}", m.to_uci_string(), nodes);
            total += nodes;
        }

        pos.unmake_move(m, info);
    }

    println!("\nTotal: {}", total);
    total
}

#[inline(always)]
fn own_king_in_check(pos: &Position, mover: crate::board::position::Color) -> bool {
    let king_bb = pos.pieces[mover as usize][PieceType::King as usize];
    debug_assert!(king_bb != 0, "perft: king missing from board");
    let king_sq = king_bb.trailing_zeros() as u8;
    pos.is_square_attacked(king_sq, mover.opposite(), pos.occupied)
}

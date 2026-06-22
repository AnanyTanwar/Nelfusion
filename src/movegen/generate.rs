use crate::board::bitboard::{Bitboard, pop_lsb};
use crate::board::position::{Color, PieceType, Position};
use crate::movegen::moves::{Move, MoveList};
use crate::movegen::tables::KNIGHT_ATTACKS;

pub fn generate_knight_moves(pos: &Position, list: &mut MoveList) {
    let us = pos.side_to_move;
    let mut knights = pos.pieces[us as usize][PieceType::Knight as usize];
    let own_pieces = pos.occupied_by[us as usize];

    while knights != 0 {
        let from = pop_lsb(&mut knights);
        let mut targets = KNIGHT_ATTACKS[from as usize] & !own_pieces;

        while targets != 0 {
            let to = pop_lsb(&mut targets);
            list.push(Move::new(from, to));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_moves_startpos() {
        let pos = Position::startpos();
        let mut list = MoveList::new();
        generate_knight_moves(&pos, &mut list);
        // Each side starts with 2 knights, each has exactly 2 legal-looking
        // (pseudo-legal) moves from the back rank in the starting position.
        assert_eq!(list.len(), 4);
    }

    #[test]
    fn test_knight_moves_center() {
        let pos = Position::from_fen("8/8/8/4N3/8/8/8/8 w - - 0 1").unwrap();
        let mut list = MoveList::new();
        generate_knight_moves(&pos, &mut list);
        // A lone knight in the center of an empty board has all 8 attacks available.
        assert_eq!(list.len(), 8);
    }
}

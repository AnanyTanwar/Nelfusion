use crate::board::bitboard::{Bitboard, pop_lsb};
use crate::board::position::{Color, PieceType, Position};
use crate::movegen::moves::{Move, MoveList};
use crate::movegen::tables::KING_ATTACKS;
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

pub fn generate_king_moves(pos: &Position, list: &mut MoveList) {
    let us = pos.side_to_move;
    let king_bb = pos.pieces[us as usize][PieceType::King as usize];
    let own_pieces = pos.occupied_by[us as usize];

    if king_bb == 0 {
        return;
    }

    let from = king_bb.trailing_zeros() as u8;
    let mut targets = KING_ATTACKS[from as usize] & !own_pieces;

    while targets != 0 {
        let to = pop_lsb(&mut targets);
        list.push(Move::new(from, to));
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
        assert_eq!(list.len(), 4);
    }

    #[test]
    fn test_knight_moves_center() {
        let pos = Position::from_fen("8/8/8/4N3/8/8/8/8 w - - 0 1").unwrap();
        let mut list = MoveList::new();
        generate_knight_moves(&pos, &mut list);
        assert_eq!(list.len(), 8);
    }

    #[test]
    fn test_king_moves_startpos() {
        let pos = Position::startpos();
        let mut list = MoveList::new();
        generate_king_moves(&pos, &mut list);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_king_moves_center() {
        let pos = Position::from_fen("8/8/8/4K3/8/8/8/8 w - - 0 1").unwrap();
        let mut list = MoveList::new();
        generate_king_moves(&pos, &mut list);
        assert_eq!(list.len(), 8);
    }
}

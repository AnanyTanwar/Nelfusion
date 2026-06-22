use crate::board::bitboard::pop_lsb;
use crate::board::position::{PieceType, Position};
use crate::movegen::magic::MAGIC_TABLES;
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

pub fn generate_bishop_moves(pos: &Position, list: &mut MoveList) {
    let us = pos.side_to_move;
    let mut bishops = pos.pieces[us as usize][PieceType::Bishop as usize];
    let own_pieces = pos.occupied_by[us as usize];
    let occupied = pos.occupied;

    while bishops != 0 {
        let from = pop_lsb(&mut bishops);
        let mut targets = MAGIC_TABLES.bishop_attacks(from as usize, occupied) & !own_pieces;

        while targets != 0 {
            let to = pop_lsb(&mut targets);
            list.push(Move::new(from, to));
        }
    }
}

pub fn generate_rook_moves(pos: &Position, list: &mut MoveList) {
    let us = pos.side_to_move;
    let mut rooks = pos.pieces[us as usize][PieceType::Rook as usize];
    let own_pieces = pos.occupied_by[us as usize];
    let occupied = pos.occupied;

    while rooks != 0 {
        let from = pop_lsb(&mut rooks);
        let mut targets = MAGIC_TABLES.rook_attacks(from as usize, occupied) & !own_pieces;

        while targets != 0 {
            let to = pop_lsb(&mut targets);
            list.push(Move::new(from, to));
        }
    }
}

pub fn generate_queen_moves(pos: &Position, list: &mut MoveList) {
    let us = pos.side_to_move;
    let mut queens = pos.pieces[us as usize][PieceType::Queen as usize];
    let own_pieces = pos.occupied_by[us as usize];
    let occupied = pos.occupied;

    while queens != 0 {
        let from = pop_lsb(&mut queens);
        let bishop_part = MAGIC_TABLES.bishop_attacks(from as usize, occupied);
        let rook_part = MAGIC_TABLES.rook_attacks(from as usize, occupied);
        let mut targets = (bishop_part | rook_part) & !own_pieces;

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

    #[test]
    fn test_bishop_moves_center_open_board() {
        let pos = Position::from_fen("8/8/8/4B3/8/8/8/8 w - - 0 1").unwrap();
        let mut list = MoveList::new();
        generate_bishop_moves(&pos, &mut list);
        assert_eq!(list.len(), 13);
    }

    #[test]
    fn test_rook_moves_corner_open_board() {
        let pos = Position::from_fen("R7/8/8/8/8/8/8/8 w - - 0 1").unwrap();
        let mut list = MoveList::new();
        generate_rook_moves(&pos, &mut list);
        assert_eq!(list.len(), 14);
    }

    #[test]
    fn test_queen_moves_center_open_board() {
        let pos = Position::from_fen("8/8/8/4Q3/8/8/8/8 w - - 0 1").unwrap();
        let mut list = MoveList::new();
        generate_queen_moves(&pos, &mut list);
        assert_eq!(list.len(), 27);
    }
}

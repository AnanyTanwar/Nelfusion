use crate::board::bitboard::pop_lsb;
use crate::board::position::{
    BK_CASTLE, BQ_CASTLE, Color, PieceType, Position, WK_CASTLE, WQ_CASTLE,
};
use crate::movegen::magic::MAGIC_TABLES;
use crate::movegen::moves::{Move, MoveList, PromoPiece};
use crate::movegen::tables::{
    KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS_BLACK, PAWN_ATTACKS_WHITE,
};

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

pub fn generate_pawn_moves(pos: &Position, list: &mut MoveList) {
    let us = pos.side_to_move;
    let mut pawns = pos.pieces[us as usize][PieceType::Pawn as usize];
    let enemies = pos.occupied_by[us.opposite() as usize];
    let empty = !pos.occupied;

    let (push_offset, start_rank, promo_rank, attack_table) = if us == Color::White {
        (8, 1, 7, &PAWN_ATTACKS_WHITE)
    } else {
        (-8, 6, 0, &PAWN_ATTACKS_BLACK)
    };

    while pawns != 0 {
        let from = pop_lsb(&mut pawns);
        let from_rank = from / 8;

        // Single push
        let to_push = (from as i32 + push_offset) as u8;
        if to_push < 64 && (empty >> to_push) & 1 != 0 {
            if to_push / 8 == promo_rank {
                push_promotions(list, from, to_push);
            } else {
                list.push(Move::new(from, to_push));

                // Double push
                if from_rank == start_rank {
                    let to_double = (from as i32 + push_offset * 2) as u8;
                    if to_double < 64 && (empty >> to_double) & 1 != 0 {
                        list.push(Move::new(from, to_double));
                    }
                }
            }
        }

        // Captures
        let mut attacks = attack_table[from as usize] & enemies;
        while attacks != 0 {
            let to = pop_lsb(&mut attacks);
            if to / 8 == promo_rank {
                push_promotions(list, from, to);
            } else {
                list.push(Move::new(from, to));
            }
        }

        // En passant
        if let Some(ep_sq) = pos.en_passant {
            if (attack_table[from as usize] >> ep_sq) & 1 != 0 {
                list.push(Move::new_en_passant(from, ep_sq));
            }
        }
    }
}

pub fn generate_castling_moves(pos: &Position, list: &mut MoveList) {
    let us = pos.side_to_move;
    let occupied = pos.occupied;

    if us == Color::White {
        if pos.castling_rights & WK_CASTLE != 0 {
            // e1=4, f1=5, g1=6, h1=7 — squares between king and rook must be empty
            if (occupied >> 5) & 1 == 0 && (occupied >> 6) & 1 == 0 {
                list.push(Move::new_castling(4, 6));
            }
        }
        if pos.castling_rights & WQ_CASTLE != 0 {
            // e1=4, d1=3, c1=2, b1=1, a1=0
            if (occupied >> 1) & 1 == 0 && (occupied >> 2) & 1 == 0 && (occupied >> 3) & 1 == 0 {
                list.push(Move::new_castling(4, 2));
            }
        }
    } else {
        if pos.castling_rights & BK_CASTLE != 0 {
            // e8=60, f8=61, g8=62, h8=63
            if (occupied >> 61) & 1 == 0 && (occupied >> 62) & 1 == 0 {
                list.push(Move::new_castling(60, 62));
            }
        }
        if pos.castling_rights & BQ_CASTLE != 0 {
            // e8=60, d8=59, c8=58, b8=57, a8=56
            if (occupied >> 57) & 1 == 0 && (occupied >> 58) & 1 == 0 && (occupied >> 59) & 1 == 0 {
                list.push(Move::new_castling(60, 58));
            }
        }
    }
}

fn push_promotions(list: &mut MoveList, from: u8, to: u8) {
    list.push(Move::new_promotion(from, to, PromoPiece::Queen));
    list.push(Move::new_promotion(from, to, PromoPiece::Rook));
    list.push(Move::new_promotion(from, to, PromoPiece::Bishop));
    list.push(Move::new_promotion(from, to, PromoPiece::Knight));
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

    #[test]
    fn test_pawn_moves_startpos_white() {
        let pos = Position::startpos();
        let mut list = MoveList::new();
        generate_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 16);
    }

    #[test]
    fn test_pawn_promotion() {
        let pos = Position::from_fen("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
        let mut list = MoveList::new();
        generate_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 4);
    }

    #[test]
    fn test_pawn_en_passant() {
        let pos = Position::from_fen("8/8/8/8/Pp6/8/8/8 b - a3 0 1").unwrap();
        let mut list = MoveList::new();
        generate_pawn_moves(&pos, &mut list);
        let has_ep = list.as_slice().iter().any(|m| m.to() == 16);
        assert!(has_ep);
    }

    #[test]
    fn test_castling_startpos_none_available() {
        let pos = Position::startpos();
        let mut list = MoveList::new();
        generate_castling_moves(&pos, &mut list);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_castling_kingside_available() {
        let pos = Position::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let mut list = MoveList::new();
        generate_castling_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }
}

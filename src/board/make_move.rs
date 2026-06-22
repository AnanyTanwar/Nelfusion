use crate::board::bitboard::{clear_bit, set_bit};
use crate::board::position::{Color, PieceType, Position};
use crate::movegen::moves::{Move, MoveType, PromoPiece};

#[derive(Copy, Clone, Debug)]
pub struct UnmakeInfo {
    pub captured_piece: Option<PieceType>,
    pub was_en_passant: bool,
    pub prev_castling_rights: u8,
    pub prev_en_passant: Option<u8>,
    pub prev_halfmove_clock: u16,
}

impl Position {
    pub fn make_move(&mut self, m: Move) -> UnmakeInfo {
        let us = self.side_to_move;
        let them = us.opposite();
        let from = m.from();
        let to = m.to();
        let move_type = m.move_type();

        let moving_pt = self
            .piece_at(from)
            .map(|(_, pt)| pt)
            .expect("make_move called with no piece on from-square");

        let is_en_passant = move_type == MoveType::EnPassant;

        let captured_piece = if is_en_passant {
            Some(PieceType::Pawn)
        } else {
            self.piece_at(to).map(|(_, pt)| pt)
        };

        let info = UnmakeInfo {
            captured_piece,
            was_en_passant: is_en_passant,
            prev_castling_rights: self.castling_rights,
            prev_en_passant: self.en_passant,
            prev_halfmove_clock: self.halfmove_clock,
        };

        if is_en_passant {
            let captured_sq = if us == Color::White { to - 8 } else { to + 8 };
            self.pieces[them as usize][PieceType::Pawn as usize] = clear_bit(
                self.pieces[them as usize][PieceType::Pawn as usize],
                captured_sq,
            );
        } else if let Some(captured_pt) = captured_piece {
            self.pieces[them as usize][captured_pt as usize] =
                clear_bit(self.pieces[them as usize][captured_pt as usize], to);
        }

        self.pieces[us as usize][moving_pt as usize] =
            clear_bit(self.pieces[us as usize][moving_pt as usize], from);

        if move_type == MoveType::Promotion {
            let promo_pt = match m.promo_piece() {
                PromoPiece::Knight => PieceType::Knight,
                PromoPiece::Bishop => PieceType::Bishop,
                PromoPiece::Rook => PieceType::Rook,
                PromoPiece::Queen => PieceType::Queen,
            };
            self.pieces[us as usize][promo_pt as usize] =
                set_bit(self.pieces[us as usize][promo_pt as usize], to);
        } else {
            self.pieces[us as usize][moving_pt as usize] =
                set_bit(self.pieces[us as usize][moving_pt as usize], to);
        }

        if move_type == MoveType::Castling {
            let (rook_from, rook_to) = match to {
                6 => (7, 5),
                2 => (0, 3),
                62 => (63, 61),
                58 => (56, 59),
                _ => unreachable!("castling move with invalid destination square"),
            };
            self.pieces[us as usize][PieceType::Rook as usize] = clear_bit(
                self.pieces[us as usize][PieceType::Rook as usize],
                rook_from,
            );
            self.pieces[us as usize][PieceType::Rook as usize] =
                set_bit(self.pieces[us as usize][PieceType::Rook as usize], rook_to);
        }

        self.castling_rights &= !castling_bits_lost(from, to);

        self.en_passant = None;
        if moving_pt == PieceType::Pawn {
            let from_rank = from / 8;
            let to_rank = to / 8;
            let rank_diff = if to_rank > from_rank {
                to_rank - from_rank
            } else {
                from_rank - to_rank
            };
            if rank_diff == 2 {
                let ep_sq = if us == Color::White {
                    from + 8
                } else {
                    from - 8
                };
                self.en_passant = Some(ep_sq);
            }
        }

        if moving_pt == PieceType::Pawn || info.captured_piece.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        self.side_to_move = them;
        if us == Color::Black {
            self.fullmove_number += 1;
        }

        self.update_occupancy();
        info
    }
}

fn castling_bits_lost(from: u8, to: u8) -> u8 {
    use crate::board::position::{BK_CASTLE, BQ_CASTLE, WK_CASTLE, WQ_CASTLE};

    let mut lost = 0u8;
    for sq in [from, to] {
        lost |= match sq {
            4 => WK_CASTLE | WQ_CASTLE,
            0 => WQ_CASTLE,
            7 => WK_CASTLE,
            60 => BK_CASTLE | BQ_CASTLE,
            56 => BQ_CASTLE,
            63 => BK_CASTLE,
            _ => 0,
        };
    }
    lost
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::position::{BK_CASTLE, BQ_CASTLE, WK_CASTLE, WQ_CASTLE};
    use crate::movegen::moves::Move;

    #[test]
    fn test_make_move_simple_push() {
        let mut pos = Position::startpos();
        let m = Move::new(12, 28);
        pos.make_move(m);

        assert_eq!(pos.piece_at(12), None);
        assert_eq!(pos.piece_at(28), Some((Color::White, PieceType::Pawn)));
        assert_eq!(pos.side_to_move, Color::Black);
        assert_eq!(pos.en_passant, Some(20));
    }

    #[test]
    fn test_make_move_capture() {
        let mut pos = Position::from_fen("8/1k6/8/8/4n3/8/4R3/7K w - - 0 1").unwrap();
        let m = Move::new(12, 28);
        let info = pos.make_move(m);

        assert_eq!(info.captured_piece, Some(PieceType::Knight));
        assert_eq!(pos.piece_at(28), Some((Color::White, PieceType::Rook)));
        assert_eq!(pos.piece_at(12), None);
        assert_eq!(pos.side_to_move, Color::Black);
    }

    #[test]
    fn test_make_move_en_passant() {
        let mut pos = Position::from_fen("8/8/8/8/Pp6/8/8/8 b - a3 0 1").unwrap();
        let m = Move::new_en_passant(25, 16);
        let info = pos.make_move(m);

        assert_eq!(info.captured_piece, Some(PieceType::Pawn));
        assert!(info.was_en_passant);
        assert_eq!(pos.piece_at(16), Some((Color::Black, PieceType::Pawn)));
        assert_eq!(pos.piece_at(24), None);
        assert_eq!(pos.piece_at(25), None);
    }

    #[test]
    fn test_make_move_castling_kingside() {
        let mut pos = Position::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let m = Move::new_castling(4, 6);
        pos.make_move(m);

        assert_eq!(pos.piece_at(6), Some((Color::White, PieceType::King)));
        assert_eq!(pos.piece_at(5), Some((Color::White, PieceType::Rook)));
        assert_eq!(pos.piece_at(7), None);
        assert_eq!(pos.piece_at(4), None);
    }

    #[test]
    fn test_make_move_promotion() {
        let mut pos = Position::from_fen("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
        let m = Move::new_promotion(48, 56, PromoPiece::Queen);
        pos.make_move(m);

        assert_eq!(pos.piece_at(56), Some((Color::White, PieceType::Queen)));
        assert_eq!(pos.piece_at(48), None);
    }

    #[test]
    fn test_castling_rights_lost_on_king_move() {
        let mut pos = Position::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let m = Move::new(4, 12);
        pos.make_move(m);

        assert_eq!(pos.castling_rights & (WK_CASTLE | WQ_CASTLE), 0);
        assert_ne!(pos.castling_rights & (BK_CASTLE | BQ_CASTLE), 0);
    }
}

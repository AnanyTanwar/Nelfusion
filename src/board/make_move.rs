use crate::board::bitboard::{clear_bit, set_bit};
use crate::board::position::{Color, PieceType, Position};
use crate::movegen::moves::{Move, MoveType, PromoPiece};

#[derive(Copy, Clone, Debug)]
pub struct UnmakeInfo {
    pub captured_piece: Option<PieceType>,
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

        let moving_pt = self
            .piece_at(from)
            .map(|(_, pt)| pt)
            .expect("make_move called with no piece on from-square");

        let info = UnmakeInfo {
            captured_piece: self.piece_at(to).map(|(_, pt)| pt),
            prev_castling_rights: self.castling_rights,
            prev_en_passant: self.en_passant,
            prev_halfmove_clock: self.halfmove_clock,
        };

        // Remove captured piece (normal capture only, not en passant — handled separately below)
        if m.move_type() != MoveType::EnPassant {
            if let Some(captured_pt) = info.captured_piece {
                self.pieces[them as usize][captured_pt as usize] =
                    clear_bit(self.pieces[them as usize][captured_pt as usize], to);
            }
        }

        // Move the piece: clear from-square, set to-square
        self.pieces[us as usize][moving_pt as usize] =
            clear_bit(self.pieces[us as usize][moving_pt as usize], from);
        self.pieces[us as usize][moving_pt as usize] =
            set_bit(self.pieces[us as usize][moving_pt as usize], to);

        // Reset halfmove clock on pawn move or capture, otherwise increment
        if moving_pt == PieceType::Pawn || info.captured_piece.is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // En passant target square only persists for one ply
        self.en_passant = None;

        self.side_to_move = them;
        if us == Color::Black {
            self.fullmove_number += 1;
        }

        self.update_occupancy();
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::movegen::moves::Move;

    #[test]
    fn test_make_move_simple_push() {
        let mut pos = Position::startpos();
        let m = Move::new(12, 28); // e2-e4
        pos.make_move(m);

        assert_eq!(pos.piece_at(12), None);
        assert_eq!(pos.piece_at(28), Some((Color::White, PieceType::Pawn)));
        assert_eq!(pos.side_to_move, Color::Black);
    }

    #[test]
    fn test_make_move_capture() {
        let mut pos = Position::from_fen("8/1k6/8/8/4n3/8/4R3/7K w - - 0 1").unwrap();
        let m = Move::new(12, 28); // rook e2 captures knight on e4
        let info = pos.make_move(m);

        assert_eq!(info.captured_piece, Some(PieceType::Knight));
        assert_eq!(pos.piece_at(28), Some((Color::White, PieceType::Rook)));
        assert_eq!(pos.piece_at(12), None); // e2 empty
        assert_eq!(pos.side_to_move, Color::Black);
    }
}

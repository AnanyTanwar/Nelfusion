use crate::board::position::{Position, Color, PieceType};
use crate::board::bitboard::{set_bit, EMPTY};
use crate::board::position::{WK_CASTLE, WQ_CASTLE, BK_CASTLE, BQ_CASTLE};

impl Position {
    /// Parses a FEN string into a Position.
    /// Example: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    pub fn from_fen(fen: &str) -> Result<Position, String> {
        let mut pos = Position::empty();
        pos.pieces = [[EMPTY; 6]; 2]; // clear default startpos-style empty board

        let parts: Vec<&str> = fen.trim().split_whitespace().collect();
        if parts.len() < 4 {
            return Err(format!("Invalid FEN: expected at least 4 fields, got {}", parts.len()));
        }

        // 1. Piece placement
        let ranks: Vec<&str> = parts[0].split('/').collect();
        if ranks.len() != 8 {
            return Err(format!("Invalid FEN: expected 8 ranks, got {}", ranks.len()));
        }

        for (rank_from_top, rank_str) in ranks.iter().enumerate() {
            let rank = 7 - rank_from_top; // FEN rank 8 is listed first
            let mut file = 0u8;

            for c in rank_str.chars() {
                if let Some(skip) = c.to_digit(10) {
                    file += skip as u8;
                } else {
                    let sq = (rank as u8) * 8 + file;
                    let color = if c.is_uppercase() { Color::White } else { Color::Black };
                    let piece_type = match c.to_ascii_lowercase() {
                        'p' => PieceType::Pawn,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'r' => PieceType::Rook,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        other => return Err(format!("Invalid piece character: {}", other)),
                    };
                    pos.pieces[color as usize][piece_type as usize] =
                        set_bit(pos.pieces[color as usize][piece_type as usize], sq);
                    file += 1;
                }
            }
        }

        // 2. Side to move
        pos.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            other => return Err(format!("Invalid side to move: {}", other)),
        };

        // 3. Castling rights
        pos.castling_rights = 0;
        if parts[2] != "-" {
            for c in parts[2].chars() {
                match c {
                    'K' => pos.castling_rights |= WK_CASTLE,
                    'Q' => pos.castling_rights |= WQ_CASTLE,
                    'k' => pos.castling_rights |= BK_CASTLE,
                    'q' => pos.castling_rights |= BQ_CASTLE,
                    other => return Err(format!("Invalid castling char: {}", other)),
                }
            }
        }

        // 4. En passant square
        pos.en_passant = if parts[3] == "-" {
            None
        } else {
            Some(square_from_str(parts[3])?)
        };

        // 5. Halfmove clock (optional in some FENs)
        pos.halfmove_clock = parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);

        // 6. Fullmove number (optional)
        pos.fullmove_number = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(1);

        pos.update_occupancy();
        Ok(pos)
    }

    /// Converts the Position back into a FEN string.
    pub fn to_fen(&self) -> String {
        let mut s = String::new();

        for rank_from_top in 0..8 {
            let rank = 7 - rank_from_top;
            let mut empty_count = 0;

            for file in 0..8 {
                let sq = rank * 8 + file;
                match self.piece_at(sq) {
                    None => empty_count += 1,
                    Some((color, piece_type)) => {
                        if empty_count > 0 {
                            s.push_str(&empty_count.to_string());
                            empty_count = 0;
                        }
                        let c = match piece_type {
                            PieceType::Pawn => 'p',
                            PieceType::Knight => 'n',
                            PieceType::Bishop => 'b',
                            PieceType::Rook => 'r',
                            PieceType::Queen => 'q',
                            PieceType::King => 'k',
                        };
                        s.push(if color == Color::White { c.to_ascii_uppercase() } else { c });
                    }
                }
            }
            if empty_count > 0 {
                s.push_str(&empty_count.to_string());
            }
            if rank_from_top != 7 {
                s.push('/');
            }
        }

        s.push(' ');
        s.push(if self.side_to_move == Color::White { 'w' } else { 'b' });

        s.push(' ');
        if self.castling_rights == 0 {
            s.push('-');
        } else {
            if self.castling_rights & WK_CASTLE != 0 { s.push('K'); }
            if self.castling_rights & WQ_CASTLE != 0 { s.push('Q'); }
            if self.castling_rights & BK_CASTLE != 0 { s.push('k'); }
            if self.castling_rights & BQ_CASTLE != 0 { s.push('q'); }
        }

        s.push(' ');
        match self.en_passant {
            None => s.push('-'),
            Some(sq) => s.push_str(&square_to_str(sq)),
        }

        s.push(' ');
        s.push_str(&self.halfmove_clock.to_string());
        s.push(' ');
        s.push_str(&self.fullmove_number.to_string());

        s
    }
}

/// Converts a square string like "e4" into a square index.
fn square_from_str(s: &str) -> Result<u8, String> {
    if s.len() != 2 {
        return Err(format!("Invalid square: {}", s));
    }
    let bytes = s.as_bytes();
    let file = bytes[0];
    let rank = bytes[1];
    if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
        return Err(format!("Invalid square: {}", s));
    }
    let file_idx = file - b'a';
    let rank_idx = rank - b'1';
    Ok(rank_idx * 8 + file_idx)
}

/// Converts a square index into algebraic notation like "e4".
fn square_to_str(sq: u8) -> String {
    let file = (sq % 8) as u8;
    let rank = (sq / 8) as u8;
    format!("{}{}", (b'a' + file) as char, (b'1' + rank) as char)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::bitboard::pop_count;

    #[test]
    fn test_startpos_fen_roundtrip() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let pos = Position::from_fen(fen).unwrap();
        assert_eq!(pop_count(pos.occupied), 32);
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn test_fen_with_en_passant() {
        let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2";
        let pos = Position::from_fen(fen).unwrap();
        assert_eq!(pos.en_passant, Some(square_from_str("e6").unwrap()));
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn test_fen_no_castling() {
        let fen = "8/8/8/4k3/4K3/8/8/8 w - - 5 10";
        let pos = Position::from_fen(fen).unwrap();
        assert_eq!(pos.castling_rights, 0);
        assert_eq!(pos.halfmove_clock, 5);
        assert_eq!(pos.fullmove_number, 10);
    }
}
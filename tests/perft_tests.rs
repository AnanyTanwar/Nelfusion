use nelfusion::board::position::Position;
use nelfusion::movegen::perft::perft;

#[test]
fn perft_startpos() {
    let mut pos = Position::startpos();
    assert_eq!(perft(&mut pos, 1), 20);
    assert_eq!(perft(&mut pos, 2), 400);
    assert_eq!(perft(&mut pos, 3), 8_902);
    assert_eq!(perft(&mut pos, 4), 197_281);
    assert_eq!(perft(&mut pos, 5), 4_865_609);
}

#[test]
fn perft_kiwipete() {
    let mut pos =
        Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    assert_eq!(perft(&mut pos, 1), 48);
    assert_eq!(perft(&mut pos, 2), 2_039);
    assert_eq!(perft(&mut pos, 3), 97_862);
    assert_eq!(perft(&mut pos, 4), 4_085_603);
}

#[test]
fn perft_position_3() {
    let mut pos = Position::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    assert_eq!(perft(&mut pos, 1), 14);
    assert_eq!(perft(&mut pos, 2), 191);
    assert_eq!(perft(&mut pos, 3), 2_812);
    assert_eq!(perft(&mut pos, 4), 43_238);
    assert_eq!(perft(&mut pos, 5), 674_624);
}

#[test]
fn perft_position_4() {
    let mut pos =
        Position::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
            .unwrap();
    assert_eq!(perft(&mut pos, 1), 6);
    assert_eq!(perft(&mut pos, 2), 264);
    assert_eq!(perft(&mut pos, 3), 9_467);
    assert_eq!(perft(&mut pos, 4), 422_333);
}

#[test]
fn perft_position_5() {
    let mut pos =
        Position::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    assert_eq!(perft(&mut pos, 1), 44);
    assert_eq!(perft(&mut pos, 2), 1_486);
    assert_eq!(perft(&mut pos, 3), 62_379);
    assert_eq!(perft(&mut pos, 4), 2_103_487);
}

#[test]
fn perft_position_6() {
    let mut pos =
        Position::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p3/2B1P3/N1P2N2/PP1Q1PPP/R4RK1 w - - 0 10")
            .unwrap();
    assert_eq!(perft(&mut pos, 1), 44);
    assert_eq!(perft(&mut pos, 2), 1_766);
    assert_eq!(perft(&mut pos, 3), 89_890);
    assert_eq!(perft(&mut pos, 4), 3_894_594);
}

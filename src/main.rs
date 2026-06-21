mod board;
mod movegen;
mod search;
mod eval;
mod uci;

fn main() {
    println!("Eclipse a UCI chess engine");
    uci::run();
}
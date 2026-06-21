mod board;
mod movegen;
mod search;
mod eval;
mod uci;

fn main() {
    println!("Nelfusion a UCI chess engine");
    uci::run();
}

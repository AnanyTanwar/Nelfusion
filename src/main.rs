mod board;
mod eval;
mod movegen;
mod search;
mod uci;

fn main() {
    println!("Nelfusion a UCI chess engine");
    uci::run();
}

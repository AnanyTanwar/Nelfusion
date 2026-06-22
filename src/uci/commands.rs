pub fn handle_command(cmd: &str) {
    let mut parts = cmd.splitn(2, ' ');
    let token = parts.next().unwrap_or("");

    match token {
        "uci" => {
            println!("id name Nelfusion");
            println!("id author Anany Tanwar");
            println!("uciok");
        }
        "isready" => println!("readyok"),
        "ucinewgame" => {}
        "position" => {}
        "go" => {}
        "stop" => {}
        _ => {}
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        std::process::exit(1);
    };
}

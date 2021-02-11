fn main() {
    let args = std::env::args();
    let config = minigrep::Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        std::process::exit(1);
    };
}

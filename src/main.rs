mod app;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("-h")) {
        usage();
        return Ok(());
    }
    let mut a: app::App = app::App::parse_cmd_args(&args).ok_or_else(|| {
        usage();
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Check flags")
    })?;

    let mut terminal = ratatui::init();

    let result = a.run(&mut terminal);

    ratatui::restore();
    result
}

fn usage() {
    println!("vree is a smal ass file tree explorer\n");
    println!("usage: vree");
    println!("  -h see this very helpful message!");
    println!("  -c turn on colors.");
    println!("  -d <dir> specify directory.");
}

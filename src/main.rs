mod app;
use color_eyre::Result;
fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("-h")) {
        usage();
        return Ok(());
    }
    let mut a: app::App = app::App::parse_cmd_args(&args).ok_or_else(|| {
        usage();
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Check flags")
    })?;

    let terminal = ratatui::init();

    let result = a.run(terminal);

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

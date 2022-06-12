use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the package
    name: String,

    /// Package version
    version: String,

    /// Nexus Mods support
    #[clap(short, long)]
    nexus: bool,

    /// Support packaging of loose files into BSAs for the given game
    #[clap(arg_enum)]
    game: Option<Game>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Game {
    Fallout4,
    FalloutNV,
    Oblvion,
}

fn main() {
    // let args = Args::parse();
    let args = Args::parse();
    match args.game {
        Some(Game::Fallout4) => {
            println!("Fallout 4!");
        }
        Some(Game::FalloutNV) => {
            println!("Fallout NV!");
        }
        Some(Game::Oblvion) => {
            println!("Oblivion!");
        }
        None => {
            println!("Uh oh");
        }
    }
}

// Terafirma static site generator
//   Penn Bauman <me@pennbauman.com>
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;
use terafirma::SiteBuilder;

static NEW_TOML: &str = "[page]
path = \"/index.html\"
body = \"<p>Hello world!</p>\"
";


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Select custom config file, the default is 'Terafirma.toml'
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Build static site, default command if unspecified
    Build {},
    /// Clean up already build site
    Clean {},
    /// Create new configuration file in the current directory
    New {},
}


fn main() -> Result<()> {
    let cli = Cli::parse();
    let file_path = cli.config.unwrap_or(PathBuf::from("Terafirma.toml"));

    match cli.command {
        Some(Commands::Build { }) | None => {
            let mut builder = SiteBuilder::from_file(file_path)?;
            println!("Build: {:?}", builder.build());
        },
        Some(Commands::Clean { }) => {
            let builder = SiteBuilder::from_file(file_path)?;
            println!("Clean: {:?}", builder.clean());
        },
        Some(Commands::New { }) => {
            let mut file = OpenOptions::new().write(true).create_new(true).open(&file_path)?;
            write!(&mut file, "{}", NEW_TOML)?;
            println!("New: {}", file_path.display());
        },
    }
    Ok(())
}


#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

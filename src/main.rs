// Terafirma static site generator
//   Penn Bauman <me@pennbauman.com>
use std::env;
use anyhow::Result;

mod builder;
use builder::SiteBuilder;


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file_path = "terafirma.toml";
    if args.len() > 1 {
        println!("{}", &args[1]);
        file_path = &args[1];
    }

    let mut builder = SiteBuilder::from_file(file_path)?;
    println!("Build: {:?}", builder.build());

    return Ok(());
}

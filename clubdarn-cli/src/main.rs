#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

extern crate clubdarn;
extern crate serde;
extern crate serde_json;

pub mod error;
mod subcommand;
mod app;

use error::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        use error_chain::ChainedError;

        let stderr = &mut ::std::io::stderr();
        let err_msg = "Error writing to stderr";

        writeln!(stderr, "{}", e.display()).expect(err_msg);
        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = app::root().get_matches();

    match matches.subcommand() {
        ("series", Some(matches)) => subcommand::series::run(matches),
        ("artist", Some(matches)) => subcommand::artist::run(matches),
        (other, _) => Err(format!("unrecognized subcommand {}", other))?,
    }
}

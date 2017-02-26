#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

extern crate clubdarn;
extern crate serde;
extern crate serde_json;

#[cfg(feature = "library")]
extern crate id3;

pub mod error;
mod subcommand;
mod app;

use error::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;

        let stderr = &mut std::io::stderr();
        let err_msg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(err_msg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(err_msg);
        }

        // If backtrace is generated (via `RUST_BACKTRACE=1`), print it
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(err_msg);
        }

        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = app::root().get_matches();

    match matches.subcommand() {
        ("song", Some(matches)) => subcommand::song::run(matches),
        ("series", Some(matches)) => subcommand::series::run(matches),
        ("artist", Some(matches)) => subcommand::artist::run(matches),
        ("category", Some(matches)) => subcommand::category::run(matches),
        (other, _) => Err(format!("unrecognized subcommand {}", other))?,
    }
}

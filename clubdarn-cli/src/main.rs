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

    // TODO: Return Result instead of Unit
    match matches.subcommand() {
        ("series", Some(matches)) => {
            subcommand::series::run(matches);
        }
        // TODO: Put this in a separate function
        ("artist", Some(matches)) => {
            let context = app::Context::from_matches(matches)?;
            let artists = context.client.artists();

            let result = if let Some(q) = matches.value_of("starts-with") {
                    artists.starting_with(q)
                } else if let Some(q) = matches.value_of("contains") {
                    artists.containing(q)
                } else {
                    Err("Unknown state")?
                }.set_page(context.page)
                .send()?;

            context.printer.stdout(&result);
        }
        (other, _) => {
            Err(format!("Invalid command {}", other))?;
        }
    };

    Ok(())
}

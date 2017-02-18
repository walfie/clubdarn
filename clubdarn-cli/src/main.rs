#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

extern crate clubdarn;
extern crate serde;
extern crate serde_json;

pub mod error;
mod subcommand;
mod args;

use error::*;
use serde::Serialize;

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

pub struct Printer {
    pub compact: bool,
}
impl Printer {
    fn stringify<T: Serialize>(&self, t: &T) -> Result<String> {
        if self.compact {
                serde_json::to_string(t)
            } else {
                serde_json::to_string_pretty(t)
            }
            .chain_err(|| "failed to serialize JSON")
    }

    pub fn stdout<T: Serialize>(&self, t: &T) -> Result<()> {
        let s = self.stringify(t);
        Ok(println!("{}", s?))
    }
}

fn run() -> Result<()> {

    let matches = args::app().get_matches();

    let metadata = clubdarn::Metadata {
        app_ver: matches.value_of("app_ver").unwrap(),
        device_id: matches.value_of("device_id").unwrap(),
        device_nm: matches.value_of("device_nm").unwrap(),
        os_ver: matches.value_of("os_ver").unwrap(),
        serial_no: matches.value_of("serial_no"),
    };

    let client = clubdarn::Client::new(metadata)
        .chain_err(|| "unable to create client")?
        .set_default_serial_no(matches.value_of("serial"));

    let printer = Printer { compact: matches.is_present("compact_output") };

    let page = value_t!(matches, "page", i32)?;

    // TODO: Return Result instead of Unit
    match matches.subcommand() {
        ("series", Some(matches)) => {
            subcommand::series::run(client, printer, matches, page);
        }
        // TODO: Put this in a separate function
        ("artist", Some(matches)) => {
            let artists = client.artists();

            let result = if let Some(q) = matches.value_of("starts_with") {
                    artists.starting_with(q)
                } else if let Some(q) = matches.value_of("contains") {
                    artists.containing(q)
                } else {
                    Err("Unknown state")?
                }.set_page(page)
                .send()?;

            pretty_print(&result);
        }
        (other, _) => {
            Err(format!("Invalid command {}", other))?;
        }
    };

    Ok(())
}

fn pretty_print<T: Serialize>(t: &T) -> Result<()> {
    let s = serde_json::to_string_pretty(t);
    println!("{}", s?);
    Ok(())
}

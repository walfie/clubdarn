#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

extern crate clubdarn;
extern crate serde;
extern crate serde_json;

pub mod error;

use clap::{AppSettings, Arg, SubCommand};
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

fn run() -> Result<()> {
    let all_categories = (&clubdarn::category::series::CATEGORIES)
        .iter()
        .map(|c| c.id.0)
        .collect::<Vec<&str>>();

    let default_metadata = clubdarn::Metadata::default();

    let matches = app_from_crate!()
        .global_setting(AppSettings::ColoredHelp)
        .settings(&[AppSettings::SubcommandRequiredElseHelp, AppSettings::GlobalVersion])
        .arg(Arg::with_name("serial_no")
            .help("Unique ID for karaoke machine (e.g., AB316238)")
            .short("s")
            .long("serial-no")
            .takes_value(true)
            .global(true))
        .arg(Arg::with_name("page")
            .help("Page number for pagination")
            .short("p")
            .long("page")
            .takes_value(true)
            .default_value("1")
            .global(true))
        .arg(Arg::with_name("app_ver")
            .help("appVer sent to ClubDAM API")
            .long("app-ver")
            .takes_value(true)
            .default_value(default_metadata.app_ver)
            .global(true))
        .arg(Arg::with_name("device_id")
            .help("deviceId sent to ClubDAM API")
            .long("device-id")
            .takes_value(true)
            .default_value(default_metadata.device_id)
            .global(true))
        .arg(Arg::with_name("device_nm")
            .help("deviceNm sent to ClubDAM API")
            .long("device-nm")
            .takes_value(true)
            .default_value(default_metadata.device_nm)
            .global(true))
        .arg(Arg::with_name("os_ver")
            .help("osVer sent to ClubDAM API")
            .long("os-ver")
            .takes_value(true)
            .default_value(default_metadata.os_ver)
            .global(true))
        .subcommand(SubCommand::with_name("series")
            .about("List various series by category ID")
            .arg(Arg::with_name("category_id")
                .required(true)
                .index(1)
                .possible_values(&all_categories)))
        .get_matches();

    let metadata = clubdarn::Metadata {
        app_ver: &value_t!(matches, "app_ver", String)?,
        device_id: &value_t!(matches, "device_id", String)?,
        device_nm: &value_t!(matches, "device_nm", String)?,
        os_ver: &value_t!(matches, "os_ver", String)?,
        serial_no: matches.value_of("serial_no"),
    };

    let client = clubdarn::Client::new(metadata)
        .chain_err(|| "unable to create client")?
        .set_default_serial_no(matches.value_of("serial"));

    let page = value_t!(matches, "page", i32)?;

    let category_id: String;

    let mut builder = match matches.subcommand() {
        ("series", Some(matches)) => {
            category_id = value_t!(matches, "category_id", String)?;
            client.series().by_category_id(&category_id)
        }
        (other, _) => Err(format!("Invalid command {}", other))?,
    };

    let result = builder.set_page(page).send()?;

    pretty_print(&result)?;
    Ok(())
}

fn pretty_print<T: Serialize>(t: &T) -> Result<()> {
    let s = serde_json::to_string_pretty(t);
    println!("{}", s?);
    Ok(())
}

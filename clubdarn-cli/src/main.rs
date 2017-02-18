#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

extern crate clubdarn;
extern crate serde;
extern crate serde_json;

pub mod error;

use clap::{AppSettings, Arg, ArgGroup, SubCommand};
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
            .value_name("SERIAL_NO")
            .takes_value(true)
            .global(true))
        .arg(Arg::with_name("page")
            .help("Page number for pagination")
            .short("p")
            .long("page")
            .value_name("PAGE_NUMBER")
            .takes_value(true)
            .default_value("1")
            .global(true))
        .arg(Arg::with_name("app_ver")
            .help("appVer sent to ClubDAM API")
            .long("app-ver")
            .value_name("APP_VERSION")
            .takes_value(true)
            .default_value(default_metadata.app_ver)
            .global(true))
        .arg(Arg::with_name("device_id")
            .help("deviceId sent to ClubDAM API")
            .long("device-id")
            .value_name("DEVICE_ID")
            .takes_value(true)
            .default_value(default_metadata.device_id)
            .global(true))
        .arg(Arg::with_name("device_nm")
            .help("deviceNm sent to ClubDAM API")
            .long("device-nm")
            .value_name("DEVICE_NAME")
            .takes_value(true)
            .default_value(default_metadata.device_nm)
            .global(true))
        .arg(Arg::with_name("os_ver")
            .help("osVer sent to ClubDAM API")
            .long("os-ver")
            .value_name("OS_VERSION")
            .takes_value(true)
            .default_value(default_metadata.os_ver)
            .global(true))
        .subcommand(SubCommand::with_name("series")
            .about("List series by category ID")
            .arg(Arg::with_name("category_id")
                .required(true)
                .index(1)
                .value_name("CATEGORY_ID")
                .possible_values(&all_categories)))
        .subcommand(SubCommand::with_name("artists")
            .about("Find artists")
            .group(ArgGroup::with_name("filter")
                .required(true)
                .args(&["starts_with", "contains"]))
            .arg(Arg::with_name("starts_with")
                .help("Find artist names starting with <QUERY>")
                .long("starts-with")
                .value_name("QUERY")
                .takes_value(true))
            .arg(Arg::with_name("contains")
                .help("Find artist names containing <QUERY>")
                .long("contains")
                .value_name("QUERY")
                .takes_value(true)))
        .get_matches();

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

    let page = value_t!(matches, "page", i32)?;

    // TODO: Put these in separate methods
    match matches.subcommand() {
        ("series", Some(matches)) => {
            let query = matches.value_of("category_id").unwrap();
            let result = client.series().by_category_id(query).set_page(page).send()?;

            pretty_print(&result)?;
        }
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

            pretty_print(&result)?;
        }
        (other, _) => Err(format!("Invalid command {}", other))?,
    };

    Ok(())
}

fn pretty_print<T: Serialize>(t: &T) -> Result<()> {
    let s = serde_json::to_string_pretty(t);
    println!("{}", s?);
    Ok(())
}

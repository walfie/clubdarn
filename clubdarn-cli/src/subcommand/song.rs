use app;
use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};
use error::*;

pub fn app() -> App<'static, 'static> {
    SubCommand::with_name("song")
        .about("Find songs")
        .group(ArgGroup::with_name("filter")
            .required(true)
            .args(&["by-name", "artist-id", "series"]))
        .group(ArgGroup::with_name("by-name").args(&["starts-with", "contains"]))
        .arg(Arg::with_name("starts-with")
            .help("Find songs with names that start with <QUERY>")
            .long("starts-with")
            .value_name("QUERY")
            .takes_value(true))
        .arg(Arg::with_name("contains")
            .help("Find songs with names containing <QUERY>")
            .long("contains")
            .value_name("QUERY")
            .takes_value(true))
        .arg(Arg::with_name("artist-id")
            .help("Find songs by artist ID <ARTIST_ID>")
            .long("artist-id")
            .value_name("ARTIST_ID")
            .takes_value(true))
        .arg(Arg::with_name("live")
            .help("Find live performances for artist ID <ARTIST_ID>")
            .long("live")
            .value_name("ARTIST_ID")
            .requires("artist-id"))
        .arg(Arg::with_name("series")
            .help("Find songs for series <SERIES_NAME> in category <CATEGORY_ID>")
            .long("series")
            .value_name("SERIES_NAME")
            .requires("category-id")
            .takes_value(true))
        .arg(Arg::with_name("category-id")
            .help("Filter by category ID")
            .long("category-id")
            .value_name("CATEGORY_ID")
            .takes_value(true))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let context = app::Context::from_matches(matches)?;
    let songs = context.client.songs();

    let result = if let Some(q) = matches.value_of("starts-with") {
            songs.starting_with(q)
        } else if let Some(q) = matches.value_of("contains") {
            songs.containing(q)
        } else {
            Err("Unknown state")?
        }.set_page(context.page)
        .send()?;

    context.printer.stdout(&result)
}

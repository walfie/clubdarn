use app;
use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};
use error::*;

pub fn app() -> App<'static, 'static> {
    SubCommand::with_name("artist")
        .about("Find artists")
        .group(ArgGroup::with_name("filter")
            .required(true)
            .args(&["starts-with", "contains", "live"]))
        .arg(Arg::with_name("starts-with")
            .help("Find artist names starting with <QUERY>")
            .long("starts-with")
            .value_name("QUERY")
            .takes_value(true))
        .arg(Arg::with_name("contains")
            .help("Find artist names containing <QUERY>")
            .long("contains")
            .value_name("QUERY")
            .takes_value(true))
        .arg(Arg::with_name("live")
            .help("List artists with live performances")
            .long("live"))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let context = app::Context::from_matches(matches)?;
    let artists = context.client.artists();

    let result = if let Some(q) = matches.value_of("starts-with") {
            artists.starting_with(q)
        } else if let Some(q) = matches.value_of("contains") {
            artists.containing(q)
        } else if matches.is_present("live") {
            artists.live_performance()
        } else {
            Err("Unknown state")?
        }.set_page(context.page)
        .send()?;

    context.printer.stdout(&result)
}

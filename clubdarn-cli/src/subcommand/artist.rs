use app;
use app::AppExt;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use clubdarn;
use error::*;

pub fn app() -> App<'static, 'static> {
    let name = SubCommand::with_name("name")
        .about("Find artists by name")
        .arg(Arg::with_name("starts-with")
            .help("Require that the match occurs at the beginning of the artist name")
            .long("starts-with"))
        .arg(Arg::with_name("query")
            .help("The query to match on")
            .value_name("QUERY")
            .empty_values(false)
            .required(true))
        .with_global_args();

    let live = SubCommand::with_name("live")
        .about("Find artists with live performances")
        .with_global_args();

    SubCommand::with_name("artist")
        .about("Find artists")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(name)
        .subcommand(live)
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let context = app::Context::from_matches(matches)?;
    let artists = context.client.artists();

    let mut request = match matches.subcommand() {
        ("name", Some(matches)) => {
            let match_type = if matches.is_present("starts-with") {
                clubdarn::MatchType::StartsWith
            } else {
                clubdarn::MatchType::Contains
            };

            artists.by_name(matches.value_of("query").unwrap(), match_type)
        }
        ("live", Some(_)) => artists.live_performance(),
        (other, _) => Err(format!("unrecognized subcommand {}", other))?,
    };

    let result = request.set_page(context.page).send()?;

    context.printer.stdout(&result)
}

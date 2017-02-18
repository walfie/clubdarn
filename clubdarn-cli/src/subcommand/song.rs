use app;
use app::AppExt;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use clubdarn;
use error::*;

pub fn app() -> App<'static, 'static> {
    let title = SubCommand::with_name("title")
        .about("Find songs by title")
        .arg(Arg::with_name("starts-with")
            .help("Require that the match occurs at the beginning of the song title")
            .long("starts-with"))
        .arg(Arg::with_name("query")
            .help("The query to match on")
            .value_name("QUERY")
            .required(true))
        .with_global_args();

    let artist = SubCommand::with_name("artist")
        .about("Find songs by artist ID")
        .arg(Arg::with_name("starts-with")
            .help("Require that the match occurs at the beginning of the song title")
            .long("starts-with"))
        .arg(Arg::with_name("artist-id")
            .help("Artist ID")
            .value_name("ARTIST_ID")
            .required(true))
        .arg(Arg::with_name("live")
            .help("List artist's live performances")
            .long("live"))
        .with_global_args();

    SubCommand::with_name("song")
        .about("Find songs")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(title)
        .subcommand(artist)
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let context = app::Context::from_matches(matches)?;
    let songs = context.client.songs();

    let mut request = match matches.subcommand() {
        ("title", Some(matches)) => {
            let match_type = if matches.is_present("starts-with") {
                clubdarn::MatchType::StartsWith
            } else {
                clubdarn::MatchType::Contains
            };

            songs.by_title(matches.value_of("query").unwrap(), match_type)
        }
        ("artist", Some(matches)) => {
            let artist_id = value_t!(matches, "artist-id", i32)?;

            if matches.is_present("live") {
                songs.by_artist_in_category_id(artist_id, clubdarn::category::LIVE_PERFORMANCE.id.0)
            } else {
                songs.by_artist_id(artist_id)
            }
        }
        (other, _) => Err(format!("unrecognized subcommand {}", other))?,
    };

    let result = request.set_page(context.page).send()?;

    context.printer.stdout(&result)
}

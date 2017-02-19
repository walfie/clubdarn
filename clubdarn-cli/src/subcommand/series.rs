use app;
use app::AppExt;
use clap::{App, Arg, ArgMatches, SubCommand};
use clubdarn::category::series::{ANIME, TOKUSATSU, MUSIC_VIDEO};
use error::*;

pub fn app() -> App<'static, 'static> {
    SubCommand::with_name("series")
        .about("List series by type")
        .arg(Arg::with_name("series-type")
            .required(true)
            .value_name("SERIES_TYPE")
            .possible_values(&["anime", "tokusatsu", "video"]))
        .with_global_args()
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let context = app::Context::from_matches(matches)?;

    let query = match matches.value_of("series-type").unwrap() {
        "anime" => ANIME.id.0,
        "tokusatsu" => TOKUSATSU.id.0,
        "video" => MUSIC_VIDEO.id.0,
        other => Err(format!("invalid series type {}", other))?,
    };
    let result = context.client.series().by_category_id(query).set_page(context.page).send()?;

    context.printer.stdout(&result)
}

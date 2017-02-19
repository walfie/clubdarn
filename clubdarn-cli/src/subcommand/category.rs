use app;
use app::AppExt;
use clap::{App, Arg, ArgMatches, SubCommand};
use error::*;

pub fn app() -> App<'static, 'static> {
    SubCommand::with_name("category")
        .about("List categories")
        .arg(Arg::with_name("category-type")
            .required(true)
            .value_name("CATEGORY_TYPE")
            .possible_values(&["new", "series", "vocaloid", "ranking"]))
        .with_global_args()
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    use clubdarn::category::*;

    let context = app::Context::from_matches(matches)?;
    let p = context.printer;

    let category_type = matches.value_of("category-type").unwrap();

    match category_type {
        "new" => return p.stdout(&new_songs::CATEGORIES),
        "series" => return p.stdout(&series::CATEGORIES),
        "vocaloid" => return p.stdout(&vocaloid::CATEGORIES),
        "ranking" => return p.stdout(&ranking::CATEGORIES),
        other => Err(format!("unknown category {}", other))?,
    };

    Ok(())
}

use app;
use clap::{App, Arg, ArgMatches, SubCommand};
use clubdarn;
use error::*;

pub fn app() -> App<'static, 'static> {
    let all_categories = (&clubdarn::category::series::CATEGORIES)
        .iter()
        .map(|c| c.id.0)
        .collect::<Vec<&str>>();

    SubCommand::with_name("series")
        .about("List series by category ID")
        .arg(Arg::with_name("category-id")
            .required(true)
            .index(1)
            .value_name("CATEGORY_ID")
            .possible_values(&all_categories))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let context = app::Context::from_matches(matches)?;

    let query = matches.value_of("category-id").unwrap();
    let result = context.client.series().by_category_id(query).set_page(context.page).send()?;

    context.printer.stdout(&result)
}

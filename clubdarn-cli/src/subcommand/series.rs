use clap::{App, Arg, SubCommand};
use clubdarn;

pub fn app() -> App<'static, 'static> {
    let all_categories = (&clubdarn::category::series::CATEGORIES)
        .iter()
        .map(|c| c.id.0)
        .collect::<Vec<&str>>();

    SubCommand::with_name("series")
        .about("List series by category ID")
        .arg(Arg::with_name("category_id")
            .required(true)
            .index(1)
            .value_name("CATEGORY_ID")
            .possible_values(&all_categories))
}

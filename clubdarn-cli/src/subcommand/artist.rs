use clap::{App, Arg, ArgGroup, SubCommand};

pub fn app() -> App<'static, 'static> {
    SubCommand::with_name("artists")
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
            .takes_value(true))
}

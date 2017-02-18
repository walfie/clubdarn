use clap::{App, AppSettings, Arg, ArgMatches};
use clubdarn;
use error::*;
use serde::Serialize;
use serde_json;
use subcommand;

pub fn root() -> App<'static, 'static> {
    let subcommands =
        vec![subcommand::song::app(), subcommand::series::app(), subcommand::artist::app()];

    app_from_crate!()
        .about(crate_description!())
        .global_setting(AppSettings::ColoredHelp)
        .settings(&[AppSettings::SubcommandRequiredElseHelp, AppSettings::VersionlessSubcommands])
        .version_short("v")
        .subcommands(subcommands)
}

pub struct Printer {
    pub compact: bool,
}

impl Printer {
    fn stringify<T: Serialize>(&self, t: &T) -> Result<String> {
        if self.compact {
                serde_json::to_string(t)
            } else {
                serde_json::to_string_pretty(t)
            }
            .chain_err(|| "failed to serialize JSON")
    }

    pub fn stdout<T: Serialize>(&self, t: &T) -> Result<()> {
        let s = self.stringify(t);
        Ok(println!("{}", s?))
    }
}

pub struct Context<'a> {
    pub client: clubdarn::Client<'a>,
    pub printer: Printer,
    pub page: i32,
}

impl<'a> Context<'a> {
    pub fn from_matches(matches: &'a ArgMatches) -> Result<Self> {
        // Take the ArgMatches from the deepest level of subcommands
        if let (_, Some(subcommand_matches)) = matches.subcommand() {
            return Self::from_matches(subcommand_matches);
        }

        let printer = Printer { compact: matches.is_present("compact-output") };
        let page = value_t!(matches, "page", i32)?;

        let metadata = clubdarn::Metadata {
            app_ver: matches.value_of("app-ver").unwrap(),
            device_id: matches.value_of("device-id").unwrap(),
            device_nm: matches.value_of("device-nm").unwrap(),
            os_ver: matches.value_of("os-ver").unwrap(),
            serial_no: matches.value_of("serial-no"),
        };

        let client = clubdarn::Client::new(metadata).chain_err(|| "unable to create client")?;

        Ok(Context {
            client: client,
            printer: printer,
            page: page,
        })
    }
}

pub trait AppExt {
    fn with_global_args(self) -> Self;
}

impl AppExt for App<'static, 'static> {
    fn with_global_args(self) -> Self {
        let default_metadata = clubdarn::Metadata::default();

        self.arg(Arg::with_name("compact-output")
                .help("Compact JSON output without pretty-printing")
                .long("compact-output")
                .short("c")
                .global(true))
            .arg(Arg::with_name("serial-no")
                .help("Unique ID for karaoke machine (e.g., AB316238)")
                .long("serial-no")
                .short("s")
                .value_name("SERIAL_NO")
                .takes_value(true)
                .global(true))
            .arg(Arg::with_name("page")
                .help("Page number for pagination")
                .long("page")
                .short("p")
                .value_name("PAGE_NUMBER")
                .takes_value(true)
                .default_value("1")
                .global(true))
            .arg(Arg::with_name("app-ver")
                .help("appVer sent to ClubDAM API")
                .long("app-ver")
                .value_name("APP_VERSION")
                .takes_value(true)
                .default_value(default_metadata.app_ver)
                .global(true))
            .arg(Arg::with_name("device-id")
                .help("deviceId sent to ClubDAM API")
                .long("device-id")
                .value_name("DEVICE_ID")
                .takes_value(true)
                .default_value(default_metadata.device_id)
                .global(true))
            .arg(Arg::with_name("device-nm")
                .help("deviceNm sent to ClubDAM API")
                .long("device-nm")
                .value_name("DEVICE_NAME")
                .takes_value(true)
                .default_value(default_metadata.device_nm)
                .global(true))
            .arg(Arg::with_name("os-ver")
                .help("osVer sent to ClubDAM API")
                .long("os-ver")
                .value_name("OS_VERSION")
                .takes_value(true)
                .default_value(default_metadata.os_ver)
                .global(true))
    }
}

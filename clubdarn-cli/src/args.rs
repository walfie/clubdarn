use clap::{App, AppSettings, Arg};
use clubdarn;
use subcommand;

pub fn app() -> App<'static, 'static> {
    let default_metadata = clubdarn::Metadata::default();

    let app = app_from_crate!()
        .global_setting(AppSettings::ColoredHelp)
        .settings(&[AppSettings::SubcommandRequiredElseHelp, AppSettings::GlobalVersion])
        .arg(Arg::with_name("compact_output")
            .help("Compact JSON output without pretty-printing")
            .short("c")
            .long("compact-output")
            .global(true))
        .arg(Arg::with_name("serial_no")
            .help("Unique ID for karaoke machine (e.g., AB316238)")
            .short("s")
            .long("serial-no")
            .value_name("SERIAL_NO")
            .takes_value(true)
            .global(true))
        .arg(Arg::with_name("page")
            .help("Page number for pagination")
            .short("p")
            .long("page")
            .value_name("PAGE_NUMBER")
            .takes_value(true)
            .default_value("1")
            .global(true))
        .arg(Arg::with_name("app_ver")
            .help("appVer sent to ClubDAM API")
            .long("app-ver")
            .value_name("APP_VERSION")
            .takes_value(true)
            .default_value(default_metadata.app_ver)
            .global(true))
        .arg(Arg::with_name("device_id")
            .help("deviceId sent to ClubDAM API")
            .long("device-id")
            .value_name("DEVICE_ID")
            .takes_value(true)
            .default_value(default_metadata.device_id)
            .global(true))
        .arg(Arg::with_name("device_nm")
            .help("deviceNm sent to ClubDAM API")
            .long("device-nm")
            .value_name("DEVICE_NAME")
            .takes_value(true)
            .default_value(default_metadata.device_nm)
            .global(true))
        .arg(Arg::with_name("os_ver")
            .help("osVer sent to ClubDAM API")
            .long("os-ver")
            .value_name("OS_VERSION")
            .takes_value(true)
            .default_value(default_metadata.os_ver)
            .global(true));

    app.subcommand(subcommand::series::app())
        .subcommand(subcommand::artist::app())
}

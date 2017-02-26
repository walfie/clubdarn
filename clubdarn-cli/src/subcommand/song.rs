use app;
use app::AppExt;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use clubdarn;
use error::*;
#[cfg(feature = "library")]
use id3;

pub fn app() -> App<'static, 'static> {
    let title = SubCommand::with_name("title")
        .about("Find songs by title")
        .arg(Arg::with_name("starts-with")
            .help("Require that the match occurs at the beginning of the song title")
            .long("starts-with"))
        .arg(Arg::with_name("query")
            .help("The query to match on")
            .value_name("QUERY")
            .multiple(true)
            .empty_values(false)
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


    // TODO: Put in a common place (also used by root series subcommand)
    let series_categories = (&clubdarn::category::series::CATEGORIES)
        .iter()
        .map(|c| c.id.0)
        .collect::<Vec<&str>>();

    let series = SubCommand::with_name("series")
        .about("Find songs from a series")
        .arg(Arg::with_name("series-title")
            .help("The series title")
            .value_name("SERIES_TITLE")
            .multiple(true)
            .required(true))
        .arg(Arg::with_name("category-id")
            .help("Category ID of the series")
            .long("category-id")
            .value_name("CATEGORY_ID")
            .required(true)
            .possible_values(&series_categories))
        .with_global_args();

    let category = SubCommand::with_name("category")
        .about("List songs from category")
        .arg(Arg::with_name("category-id")
            .help("Category ID")
            .value_name("CATEGORY_ID")
            .required(true))
        .with_global_args();

    let id = SubCommand::with_name("id")
        .about("Find songs with ID")
        .arg(Arg::with_name("song-id")
            .help("ID without hyphen (e.g., 360715)")
            .value_name("SONG_ID")
            .multiple(true)
            .required(true))
        .with_global_args();

    let similar = SubCommand::with_name("similar")
        .about("Find songs similar to the given song ID")
        .arg(Arg::with_name("song-id")
            .help("ID without hyphen (e.g., 360715)")
            .value_name("SONG_ID")
            .required(true))
        .with_global_args();

    let exact = SubCommand::with_name("exact")
        .about("Find songs with title/artist combination")
        .arg(Arg::with_name("song-title")
            .help("Song title")
            .long("title")
            .short("t")
            .value_name("SONG_TITLE")
            .multiple(true)
            .number_of_values(1)
            .required(true))
        .arg(Arg::with_name("artist-name")
            .help("Artist name")
            .long("artist")
            .short("a")
            .value_name("ARTIST_NAME")
            .multiple(true)
            .number_of_values(1)
            .required(true))
        .with_global_args();

    #[cfg(feature = "library")]
    let library = SubCommand::with_name("library")
        .about("Find songs based on local file metadata")
        .arg(Arg::with_name("file-path")
            .help("Path to file")
            .value_name("FILE_PATH")
            .multiple(true)
            .required(true))
        .with_global_args();

    let sub = SubCommand::with_name("song")
        .about("Find songs")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(title)
        .subcommand(artist)
        .subcommand(series)
        .subcommand(category)
        .subcommand(id)
        .subcommand(similar)
        .subcommand(exact);

    #[cfg(feature = "library")]
    {
        sub.subcommand(library)
    }

    #[cfg(not(feature = "library"))]
    {
        sub
    }
}

fn collect_query(matches: &ArgMatches, arg_name: &str) -> String {
    matches.values_of(arg_name).unwrap().collect::<Vec<_>>().join(" ")
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let context = app::Context::from_matches(matches)?;
    let songs = context.client.songs();

    let query: String;

    let mut request = match matches.subcommand() {
        ("title", Some(matches)) => {
            let match_type = if matches.is_present("starts-with") {
                clubdarn::MatchType::StartsWith
            } else {
                clubdarn::MatchType::Contains
            };

            query = collect_query(matches, "query");

            songs.by_title(&query, match_type)
        }
        ("artist", Some(matches)) => {
            let artist_id = value_t!(matches, "artist-id", u32)?;

            if matches.is_present("live") {
                songs.by_artist_in_category_id(artist_id, clubdarn::category::LIVE_PERFORMANCE.id.0)
            } else {
                songs.by_artist_id(artist_id)
            }
        }
        ("series", Some(matches)) => {
            query = collect_query(matches, "series-title");
            let category_id = matches.value_of("category-id").unwrap();
            songs.by_series_in_category_id(&query, category_id)
        }
        ("category", Some(matches)) => {
            let category_id = matches.value_of("category-id").unwrap();
            songs.by_category_id(category_id)
        }
        // Looking up a song by ID uses a different request type,
        // which would cause these match arms to have a incompatible
        // types. We use an explicit returns here to avoid that.
        ("id", Some(matches)) => {
            let ids = values_t!(matches, "song-id", u32)?;
            let result = songs.by_ids(&ids).set_page(context.page).send()?;
            return context.printer.stdout(&result);
        }
        ("similar", Some(matches)) => {
            let id = value_t!(matches, "song-id", u32)?;
            let result = songs.similar_to(id).set_page(context.page).send()?;
            return context.printer.stdout(&result);
        }
        ("exact", Some(matches)) => {
            let titles = matches.values_of("song-title").unwrap();
            let artists = matches.values_of("artist-name").unwrap();

            let zipped = titles.zip(artists)
                .map(|(title, artist)| {
                    clubdarn::TitleAndArtist {
                        title: title.into(),
                        artist: artist.into(),
                    }
                })
                .collect::<Vec<_>>();

            let result = songs.by_titles_and_artists(&zipped).set_page(context.page).send()?;
            return context.printer.stdout(&result);
        }
        #[cfg(feature = "library")]
        ("library", Some(matches)) => {
            let paths = matches.values_of("file-path").unwrap();

            let meta = paths.flat_map(|p| {
                    id3::Tag::read_from_path(p)
                        .ok()
                        .and_then(|tag| if let (Some(title), Some(artist)) =
                            (tag.title(), tag.artist()) {
                            Some(clubdarn::TitleAndArtist {
                                title: title.to_string().into(),
                                artist: artist.to_string().into(),
                            })
                        } else {
                            None
                        })
                })
                .collect::<Vec<_>>();

            if meta.is_empty() {
                Err("no song metadata found")?
            }

            let result = songs.by_titles_and_artists(&meta).set_page(context.page).send()?;
            return context.printer.stdout(&result);
        }
        (other, _) => Err(format!("unrecognized subcommand {}", other))?,
    };

    let result = request.set_page(context.page).send()?;

    context.printer.stdout(&result)
}

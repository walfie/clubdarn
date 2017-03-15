#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![allow(unmounted_route)]

#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate rocket;
extern crate rocket_contrib;
extern crate clubdarn;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

pub mod error;
pub use error::*;
mod cors;
mod elastic;

use cors::Cors;
use rocket::{Route, State};
use rocket_contrib::JSON;

pub type ClientState<'a> = State<'a, clubdarn::Client<'static>>;
pub type PageResult<T> = Result<Cors<JSON<clubdarn::Paginated<T>>>>;

fn main() {
    let mut args = std::env::args().skip(1);

    let default_elastic_url = "http://localhost:9200";

    let elastic_url = args.next().unwrap_or_else(|| {
        use std::io::Write;
        let mut stderr = std::io::stderr();

        writeln!(&mut stderr,
                 "No elasticsearch URL provided, using the default `{}`",
                 default_elastic_url)
            .expect("failed to write to stderr");

        default_elastic_url.to_string()
    });

    let elastic_client = elastic::Client::new(elastic_url, "series".to_string())
        .expect("Failed to create client");

    rocket::ignite()
        .mount("/api/artists", artists::routes())
        .mount("/api/songs", songs::routes())
        .mount("/api/categories", categories::routes())
        .mount("/api/series", series::routes())
        .manage(clubdarn::Client::default().unwrap())
        .manage(elastic_client)
        .launch()
}

#[derive(FromForm)]
struct CommonParams<'a> {
    page: Option<u32>,
    serial_no: Option<&'a str>,
}

macro_rules! request {
    ($params:expr, $e:expr) => {{
        let resp = $e.set_page($params.page.unwrap_or(1))
            .set_serial_no($params.serial_no).send()?;
        Ok(Cors(JSON(resp)))
    }}
}

mod series {
    use super::*;

    pub fn routes() -> Vec<Route> {
        routes![by_title]
    }

    // Rocket doesn't ignore extra params so we have to add page/serial_no to everything...
    #[derive(FromForm)]
    #[allow(dead_code)]
    struct ByTitle<'a> {
        title: String,
        page: Option<u32>,
        serial_no: Option<&'a str>,
    }

    #[get("/?<params>")]
    fn by_title(client: State<elastic::Client>, params: ByTitle) -> PageResult<clubdarn::Series> {
        let series = client.search_series(&params.title)?;

        let page = clubdarn::Paginated {
            page: 1,
            artist_category_id: clubdarn::category::ARTIST_NAME.id.0.to_string(),
            series_category_id: Some(clubdarn::category::series::ANIME.id.0.to_string()),
            total_items: series.len() as u32,
            total_pages: 1,
            items: series,
        };

        Ok(Cors(JSON(page)))
    }
}

mod artists {
    use super::*;

    pub fn routes() -> Vec<Route> {
        routes![by_name, songs, live]
    }

    #[derive(FromForm)]
    struct ByName<'a> {
        name: String,
        starts_with: Option<bool>,
        page: Option<u32>,
        serial_no: Option<&'a str>,
    }

    #[get("/?<params>")]
    fn by_name(client: ClientState, params: ByName) -> PageResult<clubdarn::Artist> {
        let match_type = match params.starts_with {
            Some(true) => clubdarn::MatchType::StartsWith,
            _ => clubdarn::MatchType::Contains,
        };

        request!(params, client.artists().by_name(&params.name, match_type))
    }

    #[get("/live?<params>")]
    fn live(client: ClientState, params: CommonParams) -> PageResult<clubdarn::Artist> {
        request!(params, client.artists().live_performance())
    }

    #[derive(FromForm)]
    struct ByCategory<'a> {
        category_id: Option<&'a str>,
        page: Option<u32>,
        serial_no: Option<&'a str>,
    }

    #[get("/<artist_id>/songs?<params>")]
    fn songs(client: ClientState,
             artist_id: u32,
             params: ByCategory)
             -> PageResult<clubdarn::Song> {
        let category_id = params.category_id.unwrap_or(clubdarn::category::ARTIST_NAME.id.0);
        request!(params,
                 client.songs().by_artist_in_category_id(artist_id, category_id))
    }
}

mod songs {
    use super::*;

    pub fn routes() -> Vec<Route> {
        routes![by_id, by_name, similar, library, library_options]
    }

    #[derive(FromForm)]
    struct ByTitle<'a> {
        title: String,
        starts_with: Option<bool>,
        page: Option<u32>,
        serial_no: Option<&'a str>,
    }

    #[get("/?<params>")]
    fn by_name(client: ClientState, params: ByTitle) -> PageResult<clubdarn::Song> {
        let match_type = match params.starts_with {
            Some(true) => clubdarn::MatchType::StartsWith,
            _ => clubdarn::MatchType::Contains,
        };

        request!(params, client.songs().by_title(&params.title, match_type))
    }

    #[get("/<song_id>?<params>")]
    fn by_id(client: ClientState,
             song_id: u32,
             params: CommonParams)
             -> PageResult<clubdarn::Song> {
        request!(params, client.songs().by_id(song_id))
    }

    #[get("/<song_id>/similar?<params>")]
    fn similar(client: ClientState,
               song_id: u32,
               params: CommonParams)
               -> PageResult<clubdarn::Song> {
        request!(params, client.songs().similar_to(song_id))
    }

    #[allow(unused_variables)]
    #[route(OPTIONS, "/lookup?<params>")]
    fn library_options(params: CommonParams) -> Cors<()> {
        Cors(())
    }

    #[post("/lookup?<params>", format = "application/json", data = "<post_data>")]
    fn library(client: ClientState,
               post_data: JSON<Vec<clubdarn::TitleAndArtist>>,
               params: CommonParams)
               -> PageResult<clubdarn::Song> {
        if post_data.is_empty() {
            // TODO: Add a separate constructor function for `Paginated`
            Ok(Cors(JSON(clubdarn::Paginated {
                page: 1,
                artist_category_id: clubdarn::category::ARTIST_NAME.id.0.to_string(),
                series_category_id: None,
                total_items: 0,
                total_pages: 1,
                items: Vec::new(),
            })))
        } else {
            request!(params, client.songs().by_titles_and_artists(&post_data))
        }
    }
}

mod categories {
    use super::*;
    use clubdarn::Paginated;
    use clubdarn::category::{self, Category, Description, SongCategory};

    pub fn routes() -> Vec<Route> {
        routes![all, series_songs, songs, series]
    }

    #[get("/?<params>")]
    #[allow(unused_variables)]
    fn all(params: CommonParams) -> PageResult<CategoryGroup> {
        let items = CATEGORY_GROUPS;
        let page = Paginated {
            page: 1,
            artist_category_id: category::ARTIST_NAME.id.0.to_string(),
            series_category_id: None,
            total_items: items.len() as u32,
            total_pages: 1,
            items: items.to_vec(),
        };
        Ok(Cors(JSON(page)))
    }

    #[get("/<category_id>/series?<params>")]
    fn series(client: ClientState,
              category_id: &str,
              params: CommonParams)
              -> PageResult<clubdarn::Series> {
        request!(params, client.series().by_category_id(category_id))
    }

    #[get("/<category_id>/series/<series_title>/songs?<params>")]
    fn series_songs(client: ClientState,
                    category_id: &str,
                    series_title: String,
                    params: CommonParams)
                    -> PageResult<clubdarn::Song> {
        request!(params,
                 client.songs().by_series_in_category_id(&series_title, category_id))
    }

    #[get("/<category_id>/songs?<params>")]
    fn songs(client: ClientState,
             category_id: &str,
             params: CommonParams)
             -> PageResult<clubdarn::Song> {
        request!(params, client.songs().by_category_id(category_id))
    }

    #[derive(Clone, Serialize)]
    struct CategoryGroup {
        description: Description,
        categories: &'static [Category<SongCategory>],
    }
    const NEW_SONGS: CategoryGroup = CategoryGroup {
        description: Description {
            en: "New Songs",
            ja: "新曲",
        },
        categories: &category::new_songs::CATEGORIES,
    };
    const RANKING: CategoryGroup = CategoryGroup {
        description: Description {
            en: "Rankings",
            ja: "ランキング",
        },
        categories: &category::ranking::CATEGORIES,
    };
    const VOCALOID: CategoryGroup = CategoryGroup {
        description: Description {
            en: "VOCALOID",
            ja: "ボーカロイド",
        },
        categories: &category::vocaloid::CATEGORIES,
    };
    const CATEGORY_GROUPS: &'static [CategoryGroup] = &[NEW_SONGS, RANKING, VOCALOID];
}

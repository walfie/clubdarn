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

pub mod error;
pub use error::*;

use rocket::{Route, State};
use rocket_contrib::JSON;

pub type ClientState<'a> = State<'a, clubdarn::Client<'static>>;
pub type PageResult<T> = Result<JSON<clubdarn::Paginated<T>>>;

fn main() {
    rocket::ignite()
        .mount("/artists", artists::routes())
        .mount("/songs", songs::routes())
        .mount("/categories", categories::routes())
        .manage(clubdarn::Client::default().unwrap())
        .launch()
}

mod artists {
    use super::*;

    pub fn routes() -> Vec<Route> {
        routes![by_name, songs, live]
    }

    #[derive(FromForm)]
    struct ByName {
        name: String,
        starts_with: Option<bool>,
    }

    #[get("/?<params>")]
    fn by_name(client: ClientState, params: ByName) -> PageResult<clubdarn::Artist> {
        let match_type = match params.starts_with {
            Some(true) => clubdarn::MatchType::StartsWith,
            _ => clubdarn::MatchType::Contains,
        };

        let resp = client.artists().by_name(&params.name, match_type).send()?;
        Ok(JSON(resp))
    }

    #[get("/live")]
    fn live(client: ClientState) -> PageResult<clubdarn::Artist> {
        let resp = client.artists().live_performance().send()?;
        Ok(JSON(resp))
    }

    #[get("/<id>/songs")]
    fn songs(client: ClientState, id: u32) -> PageResult<clubdarn::Song> {
        let resp = client.songs().by_artist_id(id).send()?;
        Ok(JSON(resp))
    }
}

mod songs {
    use super::*;

    #[derive(FromForm)]
    struct ByTitle {
        title: String,
        starts_with: Option<bool>,
    }


    pub fn routes() -> Vec<Route> {
        routes![by_id, by_name, similar]
    }

    #[get("/<id>")]
    fn by_id(client: ClientState, id: u32) -> PageResult<clubdarn::Song> {
        let resp = client.songs().by_id(id).send()?;
        Ok(JSON(resp))
    }

    #[get("/?<params>")]
    fn by_name(client: ClientState, params: ByTitle) -> PageResult<clubdarn::Song> {
        let match_type = match params.starts_with {
            Some(true) => clubdarn::MatchType::StartsWith,
            _ => clubdarn::MatchType::Contains,
        };

        let resp = client.songs().by_title(&params.title, match_type).send()?;
        Ok(JSON(resp))
    }

    #[get("/<id>/similar")]
    fn similar(client: ClientState, id: u32) -> PageResult<clubdarn::Song> {
        let resp = client.songs().similar_to(id).send()?;
        Ok(JSON(resp))
    }
}

mod categories {
    use super::*;

    pub fn routes() -> Vec<Route> {
        routes![series_songs, songs, series]
    }

    #[get("/<category_id>/series")]
    fn series(client: ClientState, category_id: &str) -> PageResult<clubdarn::Series> {
        let resp = client.series().by_category_id(category_id).send()?;
        Ok(JSON(resp))
    }

    #[get("/<category_id>/series/<series_title>/songs")]
    fn series_songs(client: ClientState,
                    category_id: &str,
                    series_title: String)
                    -> PageResult<clubdarn::Song> {
        let resp = client.songs().by_series_in_category_id(&series_title, category_id).send()?;
        Ok(JSON(resp))
    }

    #[get("/<category_id>/songs")]
    fn songs(client: ClientState, category_id: &str) -> PageResult<clubdarn::Song> {
        let resp = client.songs().by_category_id(category_id).send()?;
        Ok(JSON(resp))
    }
}

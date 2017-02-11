#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod protocol;
mod request_builder;
mod model;
mod util;
mod client;

pub mod category;
pub use client::{Client, MatchType, TitleAndArtist};
pub use request_builder::RequestBuilder;
pub use model::{Artist, ArtistId, Song, SongId, Series, Paginated};

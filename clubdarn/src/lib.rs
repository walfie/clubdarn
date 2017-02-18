#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate reqwest;

mod protocol;
mod model;
mod util;
mod client;

pub mod category;
pub mod error;
pub use client::{Client, Metadata, MatchType, TitleAndArtist, RequestBuilder};
pub use error::{Error, Result};
pub use model::{Artist, ArtistId, Song, SongId, Series, Paginated};

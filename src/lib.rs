#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod protocol;
pub mod client;
pub mod models;

pub fn example() -> () {
    let client = client::Client::new("1.2.0", "test", "hello", "4.4.4");

    let songs = client.songs_by_title("passion flower", client::STARTS_WITH).execute();
    println!("{:?}", songs.body);

    let artists = client.artists_by_name("AIKATSU", client::STARTS_WITH).execute();
    println!("{:?}", artists.body);
}

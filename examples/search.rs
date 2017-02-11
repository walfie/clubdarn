extern crate clubdarn;
extern crate serde;
extern crate serde_json;

use serde::Serialize;

use clubdarn::*;

// Run this with `cargo run --example search`

fn main() {
    let client = Client::default();

    pretty_print(&client.songs().containing("on flower").send());

    pretty_print(&client.artists().starting_with("aikatsu").send());

    pretty_print(&client.songs().by_ids(vec![369073]).send());

    {
        let lookup = TitleAndArtist {
            title: "wake up my music",
            artist: "りさ、えいみ",
        };
        pretty_print(&client.songs().by_title_and_artist(vec![lookup]).send());
    }
}

fn pretty_print<T: Serialize>(t: &T) -> () {
    let s = serde_json::to_string_pretty(t).unwrap();
    println!("{}", s);
}

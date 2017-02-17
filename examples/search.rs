extern crate clubdarn;
extern crate serde;
extern crate serde_json;

use clubdarn::*;
use serde::Serialize;

// Run this with `cargo run --example search`

fn main() {
    let client = Client::default().unwrap();

    pretty_print(client.songs().containing("on flower").send());

    pretty_print(client.artists().starting_with("aikatsu").send());

    pretty_print(client.songs().by_id(369073).send());

    pretty_print(client.songs()
        .by_title_and_artist("wake up my music", "りさ、えいみ")
        .send());
}

fn pretty_print<T: Serialize>(t: Result<T>) -> () {
    let s = serde_json::to_string_pretty(&t.unwrap()).unwrap();
    println!("{}", s);
}

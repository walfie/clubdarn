extern crate karaok;

use karaok::*;

extern crate serde;
extern crate serde_json;

use serde::Serialize;

fn pretty<T: Serialize>(x: &T) -> String {
    serde_json::to_string_pretty(x).unwrap()
}

fn main() {
    let client = client::Client::new("1.2.0", "test", "hello", "4.4.4");

    let songs = client.songs_by_title("passion flower", client::STARTS_WITH).execute();
    println!("{}", pretty(&songs.body));

    let artists = client.artists_by_name("AIKATSU", client::STARTS_WITH).execute();
    println!("{}", pretty(&artists.body));

    /*
    // Kinda slow
    let new_anime_songs = client.new_songs_by_category(protocol::categories::NEW_SONG_ALL_SONG)
        .execute();
    let mut anime_songs = new_anime_songs.body.items;
    anime_songs.sort_by_key(|s| s.date_added.to_owned());

    println!("{:?}", anime_songs);
    */
}

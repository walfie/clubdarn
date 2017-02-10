extern crate clubdarn;

use clubdarn::*;

extern crate serde;
extern crate serde_json;

use serde::Serialize;

fn pretty<T: Serialize>(x: &T) -> String {
    serde_json::to_string_pretty(x).unwrap()
}

fn main() {
    let client = Client::new("1.2.0", "test", "hello", "4.4.4");

    let songs = client.songs_by_title("passion flower", MatchType::StartsWith).send();
    println!("{}", pretty(&songs.body));

    /*
    let artists = client.artists_by_name("AIKATSU", MatchType::StartsWith).send();
    println!("{}", pretty(&artists.body));

    let song_id = client.songs_by_ids(vec!["369073"]).send();
    println!("{}", pretty(&song_id.body));

    let lookup = TitleAndArtist {
        title: "wake up my music",
        artist: "りさ、えいみ",
    };
    let x = client.songs_by_title_and_artist(vec![lookup]).send();
    println!("{}", pretty(&x.body));

    // Kinda slow
    let new_anime_songs = client.new_songs_by_category(protocol::categories::NEW_SONG_ALL_SONG)
        .execute();
    let mut anime_songs = new_anime_songs.body.items;
    anime_songs.sort_by_key(|s| s.date_added.to_owned());

    println!("{:?}", anime_songs);
    */
}

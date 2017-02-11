extern crate clubdarn;
extern crate serde;
extern crate serde_json;

use serde::Serialize;

use clubdarn::*;

fn pretty<T: Serialize>(t: &T) -> String {
    serde_json::to_string_pretty(t).unwrap()
}

fn main() {
    let client = Client::new("1.2.0", "test", "hello", "4.4.4");

    let songs = client.similar_songs(372915).send();
    println!("{}", pretty(&songs));

    /*
    let songs = client.songs_by_title("passion flower", MatchType::StartsWith).send();
    println!("{}", pretty(&songs));

    let artists = client.artists_by_name("AIKATSU", MatchType::StartsWith).send();
    println!("{}", pretty(&artists));

    let song_id = client.songs_by_ids(vec!["369073"]).send();
    println!("{}", pretty(&song_id));

    let lookup = TitleAndArtist {
        title: "wake up my music",
        artist: "りさ、えいみ",
    };
    let x = client.songs_by_title_and_artist(vec![lookup]).send();
    println!("{}", pretty(&x));

    // Kinda slow
    let new_anime_songs = client.new_songs_by_category(protocol::categories::NEW_SONG_ALL_SONG)
        .execute();
    let mut anime_songs = new_anime_songs.items;
    anime_songs.sort_by_key(|s| s.date_added.to_owned());

    println!("{:?}", anime_songs);
    */
}

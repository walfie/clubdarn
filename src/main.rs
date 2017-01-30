extern crate hyper;
extern crate serde;
extern crate serde_json;

use hyper::Client;
use std::io::Read;
use serde_json::Map;

fn main() {
    let base_url = "https://denmoku.clubdam.com/dkdenmoku/DkDamSearchServlet";

    let client = Client::new();

    let song_name = "Passion Flower";
    let json = find_songs_by_name(
        0,
        SongMatchType::StartsWith,
        song_name,
        KaraokeMachineType::Premier
    );

    let mut resp = client.post(base_url)
        .body(&json)
        .send()
        .unwrap();

    let mut buffer = String::new();

    match resp.read_to_string(&mut buffer) {
        Ok(_) => (), // Success
        Err(e) => println!("Failed: {:?}", e)
    };

    println!("{:?}", buffer);
}

enum SongMatchType { StartsWith, Partial }
impl SongMatchType {
    fn value(self) -> &'static str {
        match self {
            SongMatchType::StartsWith => "0",
            SongMatchType::Partial => "1"
        }
    }
}

enum KaraokeMachineType { Default, Premier }
impl KaraokeMachineType {
    fn value(self) -> Option<&'static str> {
        match self {
            KaraokeMachineType::Default => None,
            KaraokeMachineType::Premier => Some("AB316238")
        }
    }
}

fn find_songs_by_name(
    page: i32,
    match_type: SongMatchType,
    song_name: &str,
    machine_type: KaraokeMachineType
) -> String {
    let page_string = page.to_string();

    let mut map = Map::new();
    map.insert("appVer".to_string(), "2.1.0");
    map.insert("deviceId".to_string(), "");
    map.insert("categoryCd".to_string(), "020000");

    map.insert("page".to_string(), &page_string);

    for machine_type_value in machine_type.value().iter() {
        map.insert("serialNo".to_string(), machine_type_value);
    };

    map.insert("songMatchType".to_string(), match_type.value());
    map.insert("songName".to_string(), song_name);

    serde_json::to_string(&map).unwrap()
}


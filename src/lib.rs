#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod protocol;
pub mod client;

pub fn example() -> String {
    let client = client::Client {
        app_ver: "1.2.0",
        device_id: "test",
        device_nm: "hello",
        os_ver: "4.4.4",
        serial_no: Some("AB316238")
    };

    client.songs_by_artist_id("1")
}


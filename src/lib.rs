#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod protocol;
pub mod client;

pub fn example() -> String {
    let mut req = protocol::DkDamSearchServletRequest {
        app_ver: "1.2.0",
        device_id: "test",
        device_nm: "hello",
        os_ver: "4.4.4",
        .. Default::default()
    };

    req
        .page(1)
        .serial_no("AB316238")
        .by_song_name("passion flower", protocol::STARTS_WITH);

    serde_json::to_string_pretty(&req).unwrap()
}


#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod protocol;

pub fn example() -> String {
    let req = protocol::DkDamSearchServletRequest {
        app_ver: "1.2.0",
        device_id: "test",
        device_nm: "hello",
        os_ver: "4.4.4",

        page: "1",
        category_cd: "020000",
        .. Default::default()
    };

    serde_json::to_string_pretty(&req).unwrap()
}


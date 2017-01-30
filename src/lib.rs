#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod protocol;

pub fn example() -> String {
    let req = protocol::DkDamSearchServletRequest {
        app_ver: "1.2.0".to_string(),
        device_id: "test".to_string(),
        device_nm: "hello".to_string(),
        os_ver: "4.4.4".to_string(),

        page: "1".to_string(),
        category_cd: "020000".to_string(),
        .. Default::default()
    };

    serde_json::to_string_pretty(&req).unwrap()
}


#![feature(proc_macro)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Default, Serialize)]
struct DkDamSearchServletRequest {
    #[serde(rename = "appVer")] app_ver: String,
    #[serde(rename = "deviceId")] device_id: String,
    #[serde(rename = "deviceNm")] device_nm: String,
    #[serde(rename = "osVer")] os_ver: String,

    #[serde(rename = "page")] page: String,
    #[serde(rename = "categoryCd")] category_cd: String,

    #[serde(rename = "serialNo", skip_serializing_if = "Option::is_none")]
    serial_no: Option<String>,

    #[serde(rename = "artistId", skip_serializing_if = "Option::is_none")]
    artist_id: Option<String>,

    #[serde(rename = "artistName", skip_serializing_if = "Option::is_none")]
    artist_name: Option<String>,
    #[serde(rename = "artist_matchType", skip_serializing_if = "Option::is_none")]
    artist_match_type: Option<String>,

    #[serde(rename = "songName", skip_serializing_if = "Option::is_none")]
    song_name: Option<String>,
    #[serde(rename = "songMatchType", skip_serializing_if = "Option::is_none")]
    song_match_type: Option<String>,

    #[serde(rename = "programTitle", skip_serializing_if = "Option::is_none")]
    program_title: Option<String>
}

fn main() {
    let req = DkDamSearchServletRequest {
        app_ver: "1.2.0".to_string(),
        device_id: "test".to_string(),
        device_nm: "hello".to_string(),
        os_ver: "4.4.4".to_string(),

        page: "1".to_string(),
        category_cd: "020000".to_string(),
        .. Default::default()
    };

    let json = serde_json::to_string_pretty(&req).unwrap();

    println!("{}", json);
}


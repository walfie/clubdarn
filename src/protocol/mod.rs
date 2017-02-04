pub mod categories;

use serde::{Deserialize, Deserializer, Serializer};
use serde::de;

fn serialize_i32_as_str<S: Serializer>(n: &i32, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str((*n).to_string().as_str())
}

fn deserialize_string_as_i32<D: Deserializer>(deserializer: D) -> Result<i32, D::Error> {
    String::deserialize(deserializer).and_then(|s| {
        s.parse::<i32>().map_err(|e| de::Error::custom(de::Unexpected::Other("non-numeric string")))
    })
}

pub const SEARCH_URL: &'static str = "https://denmoku.clubdam.com/dkdenmoku/DkDamSearchServlet";

#[derive(Default, Debug, Serialize)]
pub struct DkDamSearchServletRequest<'a> {
    #[serde(rename = "appVer")]
    pub app_ver: &'a str,
    #[serde(rename = "deviceId")]
    pub device_id: &'a str,
    #[serde(rename = "deviceNm")]
    pub device_nm: &'a str,
    #[serde(rename = "osVer")]
    pub os_ver: &'a str,
    #[serde(rename = "serialNo", skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<&'a str>,

    #[serde(rename = "page", serialize_with = "serialize_i32_as_str")]
    pub page: i32,
    #[serde(rename = "categoryCd")]
    pub category_cd: &'a str,

    #[serde(rename = "artistId", skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<&'a str>,

    #[serde(rename = "artistName", skip_serializing_if = "Option::is_none")]
    pub artist_name: Option<&'a str>,
    #[serde(rename = "artistMatchType", skip_serializing_if = "Option::is_none")]
    pub artist_match_type: Option<&'a str>,

    #[serde(rename = "songName", skip_serializing_if = "Option::is_none")]
    pub song_name: Option<&'a str>,
    #[serde(rename = "songMatchType", skip_serializing_if = "Option::is_none")]
    pub song_match_type: Option<&'a str>,

    #[serde(rename = "programTitle", skip_serializing_if = "Option::is_none")]
    pub program_title: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct DkDamSearchServletResponse {
    #[serde(rename = "searchResult")]
    search_result: Vec<DkDamSearchServletSearchResult>,
    #[serde(rename = "totalCount", deserialize_with = "deserialize_string_as_i32")]
    total_count: i32,
    #[serde(rename = "totalPage", deserialize_with = "deserialize_string_as_i32")]
    total_page: i32,
}

#[derive(Debug, Deserialize)]
pub struct DkDamSearchServletSearchResult {
    #[serde(rename = "artistId")]
    artist_id: String,
    #[serde(rename = "artistName")]
    artist_name: String,
    #[serde(rename = "distEnd")]
    dist_end: String,
    #[serde(rename = "distStart")]
    dist_start: String,
    #[serde(rename = "firstBars")]
    first_bars: String,
    #[serde(rename = "funcAnimePicture")]
    func_anime_picture: String,
    #[serde(rename = "funcPersonPicture")]
    func_person_picture: String,
    #[serde(rename = "funcRecording")]
    func_recording: String,
    #[serde(rename = "funcScore")]
    func_score: String,
    #[serde(rename = "indicationMonth")]
    indication_month: String,
    #[serde(rename = "myKey")]
    my_key: String,
    #[serde(rename = "orgKey")]
    org_key: String,
    #[serde(rename = "programTitle")]
    program_title: String,
    #[serde(rename = "reqNo")]
    req_no: String,
    #[serde(rename = "songName")]
    song_name: String,
    #[serde(rename = "titleFirstKana")]
    title_first_kana: String,
}

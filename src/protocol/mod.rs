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

#[derive(Default, Serialize)]
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

#[derive(Serialize)]
pub struct DkDamSearchServletResponse<'a> {
    #[serde(rename = "searchResult")]
    search_result: Vec<DkDamSearchServletSearchResult<'a>>,
    #[serde(rename = "totalCount", deserialize_with = "deserialize_string_as_i32")]
    total_count: i32,
    #[serde(rename = "totalPage", deserialize_with = "deserialize_string_as_i32")]
    total_page: i32,
}

#[derive(Serialize)]
pub struct DkDamSearchServletSearchResult<'a> {
    #[serde(rename = "artistId")]
    artist_id: &'a str,
    #[serde(rename = "artistName")]
    artist_name: &'a str,
    #[serde(rename = "distEnd")]
    dist_end: &'a str,
    #[serde(rename = "distStart")]
    dist_start: &'a str,
    #[serde(rename = "firstBars")]
    first_bars: &'a str,
    #[serde(rename = "funcAnimePicture")]
    func_anime_picture: &'a str,
    #[serde(rename = "funcPersonPicture")]
    func_person_picture: &'a str,
    #[serde(rename = "funcRecording")]
    func_recording: &'a str,
    #[serde(rename = "funcScore")]
    func_score: &'a str,
    #[serde(rename = "indicationMonth")]
    indication_month: &'a str,
    #[serde(rename = "myKey")]
    my_key: &'a str,
    #[serde(rename = "orgKey")]
    org_key: &'a str,
    #[serde(rename = "programTitle")]
    program_title: &'a str,
    #[serde(rename = "reqNo")]
    req_no: &'a str,
    #[serde(rename = "songName")]
    song_name: &'a str,
    #[serde(rename = "titleFirstKana")]
    title_first_kana: &'a str,
}

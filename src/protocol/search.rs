use client::ClientMetadata;
use std::borrow::Cow;
use super::*;
use super::serialize_util::*;

pub const API_URL: &'static str = "https://denmoku.clubdam.com/dkdenmoku/DkDamSearchServlet";

#[derive(Default, Debug, Serialize)]
pub struct Request<'a> {
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

impl<'a> api::Request<'a> for Request<'a> {
    type ResponseType = Response<'a>;
    fn url() -> &'a str {
        API_URL
    }

    fn from_client_metadata(meta: &ClientMetadata<'a>) -> Self {
        Request {
            app_ver: meta.app_ver,
            device_id: meta.device_id,
            device_nm: meta.device_nm,
            os_ver: meta.os_ver,
            serial_no: meta.serial_no,
            page: 1,
            ..Default::default()
        }
    }

    fn category(&self) -> Cow<'a, str> {
        self.category_cd.into()
    }

    fn get_page(&self) -> i32 {
        self.page
    }

    fn page(&self, page_num: i32) -> Self {
        Request { page: page_num, ..Default::default() }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response<'a> {
    #[serde(rename = "searchResult")]
    pub search_result: Vec<Item<'a>>,
    #[serde(rename = "totalCount", deserialize_with = "deserialize_string_as_i32")]
    pub total_count: i32,
    #[serde(rename = "totalPage", deserialize_with = "deserialize_string_as_i32")]
    pub total_page: i32,
}

impl<'a> api::Response<'a> for Response<'a> {
    type ItemType = Item<'a>;

    fn items(self) -> Vec<Item<'a>> {
        self.search_result
    }

    fn total_pages(&self) -> i32 {
        if (self.search_result.len() as i32) >= self.total_count {
            1
        } else {
            self.total_page
        }
    }

    fn total_items(&self) -> i32 {
        self.total_count
    }
}

#[derive(Debug, Deserialize)]
pub struct Item<'a> {
    #[serde(rename = "artistId")]
    pub artist_id: Cow<'a, str>,
    #[serde(rename = "artistName")]
    pub artist_name: Cow<'a, str>,
    #[serde(rename = "distEnd")]
    pub dist_end: Cow<'a, str>,
    #[serde(rename = "distStart")]
    pub dist_start: Cow<'a, str>,
    #[serde(rename = "firstBars")]
    pub first_bars: Cow<'a, str>,
    #[serde(rename = "funcAnimePicture")]
    pub func_anime_picture: Cow<'a, str>,
    #[serde(rename = "funcPersonPicture")]
    pub func_person_picture: Cow<'a, str>,
    #[serde(rename = "funcRecording")]
    pub func_recording: Cow<'a, str>,
    #[serde(rename = "funcScore")]
    pub func_score: Cow<'a, str>,
    #[serde(rename = "indicationMonth")]
    pub indication_month: Cow<'a, str>,
    #[serde(rename = "myKey")]
    pub my_key: Cow<'a, str>,
    #[serde(rename = "orgKey")]
    pub org_key: Cow<'a, str>,
    #[serde(rename = "programTitle")]
    pub program_title: Cow<'a, str>,
    #[serde(rename = "reqNo")]
    pub req_no: Cow<'a, str>,
    #[serde(rename = "songName")]
    pub song_name: Cow<'a, str>,
    #[serde(rename = "titleFirstKana")]
    pub title_first_kana: Cow<'a, str>,
}

use std::borrow::Cow;

use client;
use super::*;
use super::super::util::*;

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

    pub page: i32,
    #[serde(rename = "categoryCd")]
    pub category_cd: Cow<'a, str>,

    #[serde(rename = "artistId", skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<i32>,

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
    type ResponseType = Response;

    fn request_type() -> api::RequestType {
        api::RequestType::Json
    }

    fn url() -> &'a str {
        API_URL
    }

    fn from_client_metadata(meta: &client::Metadata<'a>) -> Self {
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

    fn set_serial_no(&mut self, serial_no: Option<&'a str>) -> &Self {
        self.serial_no = serial_no;
        self
    }

    fn category(&self) -> Option<Cow<'a, str>> {
        Some(self.category_cd.clone())
    }

    fn page(&self) -> i32 {
        self.page
    }

    fn set_page(&mut self, page_num: i32) -> &Self {
        self.page = page_num;
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "searchResult")]
    pub search_result: Vec<Item>,
    #[serde(rename = "totalCount", deserialize_with = "deserialize_string_as_i32")]
    pub total_count: i32,
    #[serde(rename = "totalPage", deserialize_with = "deserialize_string_as_i32")]
    pub total_page: i32,
}

impl api::Response for Response {
    type ItemType = Item;

    fn items(self) -> Vec<Item> {
        self.search_result
    }

    fn total_pages(&self) -> i32 {
        if (self.search_result.len() as i32) >= self.total_count {
            1
        } else {
            self.total_page
        }
    }

    fn total_items(&self) -> Option<i32> {
        Some(self.total_count)
    }
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "artistId")]
    pub artist_id: String,
    #[serde(rename = "artistName")]
    pub artist_name: String,
    #[serde(rename = "distEnd")]
    pub dist_end: String,
    #[serde(rename = "distStart")]
    pub dist_start: String,
    #[serde(rename = "firstBars")]
    pub first_bars: String,
    #[serde(rename = "funcAnimePicture")]
    pub func_anime_picture: String,
    #[serde(rename = "funcPersonPicture")]
    pub func_person_picture: String,
    #[serde(rename = "funcRecording")]
    pub func_recording: String,
    #[serde(rename = "funcScore")]
    pub func_score: String,
    #[serde(rename = "indicationMonth")]
    pub indication_month: String,
    #[serde(rename = "myKey")]
    pub my_key: String,
    #[serde(rename = "orgKey")]
    pub org_key: String,
    #[serde(rename = "programTitle")]
    pub program_title: String,
    #[serde(rename = "reqNo")]
    pub req_no: String,
    #[serde(rename = "songName")]
    pub song_name: String,
    #[serde(rename = "titleFirstKana")]
    pub title_first_kana: String,
}

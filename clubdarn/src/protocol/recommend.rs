use super::*;

use client;
use std::borrow::Cow;

pub const API_URL: &'static str = "https://csgw.clubdam.com/minsei/recommend/GetRecommendSongs.api";

#[derive(Debug, Serialize)]
pub struct Request<'a> {
    #[serde(rename = "compId")]
    comp_id: i8,
    #[serde(rename = "contractId")]
    contract_id: i8,
    #[serde(rename = "compAuthKey")]
    comp_auth_key: &'a str,
    format: &'a str,
    #[serde(rename = "requestNoList")]
    pub request_no_list: Cow<'a, str>,
    pub serial: Option<&'a str>,
}

impl<'a> api::Request<'a> for Request<'a> {
    type ResponseType = Response;

    fn request_type() -> api::RequestType {
        api::RequestType::FormData
    }

    fn url() -> &'a str {
        API_URL
    }

    fn from_client_metadata(meta: &client::Metadata<'a>) -> Self {
        // Same values as the ones hardcoded into the Denmoku app
        Request {
            comp_id: 1,
            contract_id: 1,
            comp_auth_key: "2/Qb9R@8s*",
            format: "json",
            request_no_list: "".into(),
            serial: meta.serial_no,
        }
    }

    fn set_serial_no(&mut self, serial_no: Option<&'a str>) -> &Self {
        self.serial = serial_no;
        self
    }

    fn category(&'a self) -> Option<&'a str> {
        None
    }

    fn page(&self) -> i32 {
        1
    }

    #[allow(unused_variables)]
    fn set_page(&mut self, page_num: i32) -> &Self {
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub list: Vec<Item>,
}

impl api::Response for Response {
    type ItemType = Item;

    fn take_items(self) -> Vec<Item> {
        self.list
    }

    fn total_pages(&self) -> i32 {
        1
    }

    fn total_items(&self) -> Option<i32> {
        Some(self.list.len() as i32)
    }
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "artist")]
    pub artist: String,
    #[serde(rename = "artistCode")]
    pub artist_code: String,
    #[serde(rename = "contents")]
    pub contents: String,
    #[serde(rename = "contentsId")]
    pub contents_id: String,
    #[serde(rename = "contentsYomi")]
    pub contents_yomi: String,
    #[serde(rename = "dArtistNameYomi")]
    pub d_artist_name_yomi: String,
    #[serde(rename = "dSongNameYomi")]
    pub d_song_name_yomi: String,
    #[serde(rename = "damArtistCode")]
    pub dam_artist_code: String,
    #[serde(rename = "denmokuArtist")]
    pub denmoku_artist: String,
    #[serde(rename = "denmokuContents")]
    pub denmoku_contents: String,
    #[serde(rename = "nameYomi")]
    pub name_yomi: String,
    #[serde(rename = "requestNo")]
    pub request_no: String,
}

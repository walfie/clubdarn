use client::ClientMetadata;
use std::borrow::Cow;
use super::*;

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
    type ResponseType = Response<'a>;

    fn request_type() -> api::RequestType {
        api::RequestType::FormData
    }

    fn url() -> &'a str {
        API_URL
    }

    fn from_client_metadata(meta: &ClientMetadata<'a>) -> Self {
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

    fn set_serial_no(&mut self, serial_no: &'a str) -> &Self {
        self.serial = Some(serial_no);
        self
    }

    fn unset_serial_no(&mut self) -> &Self {
        self.serial = None;
        self
    }

    fn category(&self) -> Option<Cow<'a, str>> {
        None
    }

    fn get_page(&self) -> i32 {
        1
    }

    #[allow(unused_variables)]
    fn set_page(&mut self, page_num: i32) -> &Self {
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct Response<'a> {
    pub list: Vec<Item<'a>>,
}

impl<'a> api::Response<'a> for Response<'a> {
    type ItemType = Item<'a>;

    fn items(self) -> Vec<Item<'a>> {
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
pub struct Item<'a> {
    #[serde(rename = "artist")]
    pub artist: Cow<'a, str>,
    #[serde(rename = "artistCode")]
    pub artist_code: Cow<'a, str>,
    #[serde(rename = "contents")]
    pub contents: Cow<'a, str>,
    #[serde(rename = "contentsId")]
    pub contents_id: Cow<'a, str>,
    #[serde(rename = "contentsYomi")]
    pub contents_yomi: Cow<'a, str>,
    #[serde(rename = "dArtistNameYomi")]
    pub d_artist_name_yomi: Cow<'a, str>,
    #[serde(rename = "dSongNameYomi")]
    pub d_song_name_yomi: Cow<'a, str>,
    #[serde(rename = "damArtistCode")]
    pub dam_artist_code: Cow<'a, str>,
    #[serde(rename = "denmokuArtist")]
    pub denmoku_artist: Cow<'a, str>,
    #[serde(rename = "denmokuContents")]
    pub denmoku_contents: Cow<'a, str>,
    #[serde(rename = "nameYomi")]
    pub name_yomi: Cow<'a, str>,
    #[serde(rename = "requestNo")]
    pub request_no: Cow<'a, str>,
}

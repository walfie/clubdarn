use super::*;
use client;

pub const API_URL: &'static str = "https://denmoku.clubdam.com/dkdenmoku/DkDamIsExistServlet";

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

    #[serde(rename = "isExist")]
    pub is_exist: Vec<RequestItem<'a>>,
}

#[derive(Default, Debug, Serialize)]
pub struct RequestItem<'a> {
    #[serde(rename = "artistName", skip_serializing_if = "Option::is_none")]
    artist_name: Option<&'a str>,
    #[serde(rename = "songName", skip_serializing_if = "Option::is_none")]
    song_name: Option<&'a str>,
    #[serde(rename = "reqNo", skip_serializing_if = "Option::is_none")]
    req_no: Option<u32>,
}

impl<'a> RequestItem<'a> {
    pub fn from_id(id: u32) -> Self {
        RequestItem {
            song_name: None,
            artist_name: None,
            req_no: Some(id),
        }
    }

    pub fn from_title_and_artist(title: &'a str, artist: &'a str) -> Self {
        RequestItem {
            song_name: Some(title),
            artist_name: Some(artist),
            req_no: None,
        }
    }
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
            ..Default::default()
        }
    }

    fn category(&'a self) -> Option<&'a str> {
        None
    }

    fn set_serial_no(&mut self, serial_no: Option<&'a str>) -> &mut Self {
        self.serial_no = serial_no;
        self
    }

    fn page(&self) -> u32 {
        1
    }

    #[allow(unused_variables)]
    fn set_page(&mut self, page_num: u32) -> &mut Self {
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "isExist")]
    pub is_exist: Vec<Item>,
}

impl api::Response for Response {
    type ItemType = Item;

    fn take_items(self) -> Vec<Item> {
        self.is_exist.into_iter().filter(|item| !item.req_no.is_empty()).collect()
    }

    fn total_pages(&self) -> u32 {
        1
    }

    fn total_items(&self) -> Option<u32> {
        None
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
    #[serde(rename = "myKey")]
    pub my_key: String,
    #[serde(rename = "orgKey")]
    pub org_key: String,
    #[serde(rename = "reqNo")]
    pub req_no: String,
    #[serde(rename = "songName")]
    pub song_name: String,
}

use client::{ClientMetadata, SongLookup};
use std::borrow::Cow;
use super::*;

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
    req_no: Option<&'a str>,
}

impl<'a> From<SongLookup<'a>> for RequestItem<'a> {
    fn from(lookup: SongLookup<'a>) -> Self {
        match lookup {
            SongLookup::ByTitleAndArtist { title, artist_name } => {
                RequestItem {
                    song_name: Some(title),
                    artist_name: Some(artist_name),
                    req_no: None,
                }
            }
            SongLookup::ById(id) => {
                RequestItem {
                    song_name: None,
                    artist_name: None,
                    req_no: Some(id),
                }
            }
        }
    }
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
            ..Default::default()
        }
    }

    fn category(&self) -> Option<Cow<'a, str>> {
        None
    }

    fn get_page(&self) -> i32 {
        1
    }

    #[allow(unused_variables)]
    fn page(&self, page_num: i32) -> Self {
        // TODO: This is wrong
        Request { ..Default::default() }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response<'a> {
    #[serde(rename = "isExist")]
    pub is_exist: Vec<Item<'a>>,
}

impl<'a> api::Response<'a> for Response<'a> {
    type ItemType = Item<'a>;

    fn items(self) -> Vec<Item<'a>> {
        // figure out why below isn't working
        //self.is_exist.into_iter().filter(|item| !item.req_no.is_empty()).collect()
        self.is_exist
    }

    fn total_pages(&self) -> i32 {
        1
    }

    fn total_items(&self) -> Option<i32> {
        None
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
    #[serde(rename = "myKey")]
    pub my_key: Cow<'a, str>,
    #[serde(rename = "orgKey")]
    pub org_key: Cow<'a, str>,
    #[serde(rename = "reqNo")]
    pub req_no: Cow<'a, str>,
    #[serde(rename = "songName")]
    pub song_name: Cow<'a, str>,
}

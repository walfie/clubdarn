extern crate serde_json;
extern crate reqwest;

use std::marker::PhantomData;
use std::sync::Arc;

use model::*;
use request_builder::*;

const DEFAULT_APP_VER: &'static str = "1.2.0"; // Denmoku Mini app version
const DEFAULT_DEVICE_ID: &'static str = "";
const DEFAULT_DEVICE_NM: &'static str = env!("CARGO_PKG_NAME");
const DEFAULT_OS_VER: &'static str = env!("CARGO_PKG_VERSION");

pub struct Client<'a> {
    http: Arc<reqwest::Client>,
    meta: Metadata<'a>,
}

pub struct Metadata<'a> {
    pub app_ver: &'a str,
    pub device_id: &'a str,
    pub device_nm: &'a str,
    pub os_ver: &'a str,
    pub serial_no: Option<&'a str>,
}

impl<'a> Client<'a> {
    pub fn new(app_ver: &'a str, device_id: &'a str, device_nm: &'a str, os_ver: &'a str) -> Self {
        let meta = Metadata {
            app_ver: app_ver,
            device_id: device_id,
            device_nm: device_nm,
            os_ver: os_ver,
            serial_no: None,
        };

        Client {
            http: Arc::new(reqwest::Client::new().unwrap()),
            meta: meta,
        }
    }

    pub fn default() -> Self {
        Self::new(DEFAULT_APP_VER,
                  DEFAULT_DEVICE_ID,
                  DEFAULT_DEVICE_NM,
                  DEFAULT_OS_VER)
    }

    fn request_builder<T, U>(&self, req: T) -> RequestBuilder<'a, T, U> {
        RequestBuilder {
            http: self.http.clone(),
            request: req,
            response_item_type: PhantomData,
        }
    }

    pub fn serial_no(mut self, serial_no: Option<&'a str>) -> Self {
        self.meta.serial_no = serial_no;
        self
    }

    pub fn artists(&self) -> RequestBuilder<&Metadata, Artist> {
        self.request_builder(&self.meta)
    }

    pub fn songs(&self) -> RequestBuilder<&Metadata, Song> {
        self.request_builder(&self.meta)
    }

    pub fn series(&self) -> RequestBuilder<&Metadata, Series> {
        self.request_builder(&self.meta)
    }
}

#[derive(Debug, Serialize)]
pub struct TitleAndArtist<'a> {
    pub title: &'a str,
    pub artist: &'a str,
}

pub enum MatchType {
    StartsWith,
    Contains,
}

impl From<MatchType> for &'static str {
    fn from(mt: MatchType) -> Self {
        match mt {
            MatchType::StartsWith => "0",
            MatchType::Contains => "1",
        }
    }
}

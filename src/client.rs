extern crate serde_json;
extern crate reqwest;

use model::*;
use category;
use protocol::{api, search, exist};
use std::marker::PhantomData;
use std::sync::Arc;
use request_builder::*;

pub struct Client<'a> {
    http: Arc<reqwest::Client>,
    meta: ClientMetadata<'a>,
}

pub struct ClientMetadata<'a> {
    pub app_ver: &'a str,
    pub device_id: &'a str,
    pub device_nm: &'a str,
    pub os_ver: &'a str,
    pub serial_no: Option<&'a str>,
}

impl<'a> Client<'a> {
    pub fn new(app_ver: &'a str, device_id: &'a str, device_nm: &'a str, os_ver: &'a str) -> Self {
        let meta = ClientMetadata {
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

    fn default_request<T: api::Request<'a>>(&self) -> T {
        T::from_client_metadata(&self.meta)
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

    pub fn songs_by_artist_id(&self, id: &'a str) -> RequestBuilder<search::Request, Song> {
        let mut req = self.default_request::<search::Request>();
        req.artist_id = Some(id);
        req.category_cd = category::ARTIST_NAME.0;

        self.request_builder(req)
    }

    pub fn songs_by_title(&self,
                          title: &'a str,
                          match_type: MatchType)
                          -> RequestBuilder<search::Request, Song> {
        let mut req = self.default_request::<search::Request>();
        req.song_name = Some(title);
        req.category_cd = category::SONG_NAME.0;
        req.song_match_type = Some(match_type.into());

        self.request_builder(req)
    }

    pub fn songs_by_series(&self,
                           title: &'a str,
                           category: category::CategoryId)
                           -> RequestBuilder<search::Request, Song> {
        let mut req = self.default_request::<search::Request>();
        req.program_title = Some(title);
        req.category_cd = category.0;

        self.request_builder(req)
    }

    pub fn artists_by_name(&self,
                           name: &'a str,
                           match_type: MatchType)
                           -> RequestBuilder<search::Request, Artist> {
        let mut req = self.default_request::<search::Request>();
        req.artist_name = Some(name);
        req.category_cd = category::ARTIST_NAME.0;
        req.artist_match_type = Some(match_type.into());

        self.request_builder(req)
    }

    pub fn series_by_category(&self,
                              category: category::CategoryId)
                              -> RequestBuilder<search::Request, Series> {
        let mut req = self.default_request::<search::Request>();
        req.category_cd = category.0;

        self.request_builder(req)
    }

    pub fn new_songs_by_category(&self,
                                 category: category::CategoryId)
                                 -> RequestBuilder<search::Request, Song> {
        let mut req = self.default_request::<search::Request>();
        req.category_cd = category.0;

        self.request_builder(req)
    }

    pub fn songs_by_ids(&self, ids: Vec<&'a str>) -> RequestBuilder<exist::Request, Song> {
        let mut req = self.default_request::<exist::Request>();
        req.is_exist = ids.iter().map(|id| exist::RequestItem::from_id(id)).collect();

        self.request_builder(req)
    }

    pub fn songs_by_title_and_artist(&self,
                                     titles_and_artists: Vec<TitleAndArtist<'a>>)
                                     -> RequestBuilder<exist::Request, Song> {
        let mut req = self.default_request::<exist::Request>();
        req.is_exist = titles_and_artists.iter()
            .map(|x| exist::RequestItem::from_title_and_artist(x.title, x.artist))
            .collect();

        self.request_builder(req)
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

extern crate serde_json;
extern crate reqwest;

use models;
use protocol::{categories, SearchRequest, SearchResultsWrapper};
use protocol;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct Client<'a> {
    http: Arc<reqwest::Client>,
    default_request: SearchRequest<'a>,
}

impl<'a> Client<'a> {
    pub fn new(app_ver: &'a str, device_id: &'a str, device_nm: &'a str, os_ver: &'a str) -> Self {
        let req = SearchRequest {
            app_ver: app_ver,
            device_id: device_id,
            device_nm: device_nm,
            os_ver: os_ver,
            page: 1,
            ..Default::default()
        };

        Client {
            http: Arc::new(reqwest::Client::new().unwrap()),
            default_request: req,
        }
    }

    pub fn serial_no(mut self, serial_no: Option<&'a str>) -> Self {
        self.default_request.serial_no = serial_no;
        self
    }

    pub fn songs_by_artist_id(&self, id: &'a str) -> RequestBuilder {
        RequestBuilder {
            http: self.http.clone(),
            inner: SearchRequest {
                artist_id: Some(id),
                category_cd: categories::ARTIST_NAME.0,
                ..self.default_request
            },
        }
    }

    pub fn songs_by_title(&self, title: &'a str, match_type: MatchType) -> RequestBuilder {
        RequestBuilder {
            http: self.http.clone(),
            inner: SearchRequest {
                song_name: Some(title),
                category_cd: categories::SONG_NAME.0,
                song_match_type: Some(match_type.0),
                ..self.default_request
            },
        }
    }

    pub fn songs_by_series(&self,
                           title: &'a str,
                           category: categories::CategoryId)
                           -> RequestBuilder {
        RequestBuilder {
            http: self.http.clone(),
            inner: SearchRequest {
                program_title: Some(title),
                category_cd: category.0,
                ..self.default_request
            },
        }
    }

    pub fn artists_by_name(&self, name: &'a str, match_type: MatchType) -> RequestBuilder {
        RequestBuilder {
            http: self.http.clone(),
            inner: SearchRequest {
                artist_name: Some(name),
                category_cd: categories::ARTIST_NAME.0,
                artist_match_type: Some(match_type.0),
                ..self.default_request
            },
        }
    }
}

pub struct MatchType(pub &'static str);
pub const STARTS_WITH: MatchType = MatchType("0");
pub const CONTAINS: MatchType = MatchType("1");

pub struct RequestBuilder<'a> {
    http: Arc<reqwest::Client>,
    inner: SearchRequest<'a>, 
    // response_type: PhantomData<T>,
}

impl<'a> RequestBuilder<'a> {
    pub fn page(&mut self, page_num: i32) -> &Self {
        self.inner.page = page_num;
        self
    }

    // TODO: Handle errors
    pub fn execute(&self) -> SearchResultsWrapper<'a> {
        let json = serde_json::to_string(&self.inner).unwrap();

        self.http
            .post(protocol::SEARCH_URL)
            .body(json)
            .send()
            .unwrap()
            .json()
            .unwrap()
    }
}

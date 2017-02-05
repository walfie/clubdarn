extern crate serde_json;
extern crate reqwest;

use models::*;
use protocol::{categories, SearchRequest, SearchResultsWrapper};
use protocol;
use std::marker::PhantomData;
use std::sync::Arc;
use std::ops::Deref;

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

    pub fn songs_by_artist_id(&self, id: &'a str) -> RequestBuilder<Paginated<Song>> {
        RequestBuilder {
            http: self.http.clone(),
            response_type: PhantomData,
            inner: SearchRequest {
                artist_id: Some(id),
                category_cd: categories::ARTIST_NAME.0,
                ..self.default_request
            },
        }
    }

    pub fn songs_by_title(&self,
                          title: &'a str,
                          match_type: MatchType)
                          -> RequestBuilder<Paginated<Song>> {
        RequestBuilder {
            http: self.http.clone(),
            response_type: PhantomData,
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
                           -> RequestBuilder<Paginated<Song>> {
        RequestBuilder {
            http: self.http.clone(),
            response_type: PhantomData,
            inner: SearchRequest {
                program_title: Some(title),
                category_cd: category.0,
                ..self.default_request
            },
        }
    }

    pub fn artists_by_name(&self,
                           name: &'a str,
                           match_type: MatchType)
                           -> RequestBuilder<Paginated<Artist>> {
        RequestBuilder {
            http: self.http.clone(),
            response_type: PhantomData,
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

#[derive(Debug)]
pub struct RequestBuilder<'a, T: From<SearchResultsWrapper<'a>>> {
    http: Arc<reqwest::Client>,
    inner: SearchRequest<'a>,
    response_type: PhantomData<T>,
}

#[derive(Debug)]
pub struct Response<'a, T: From<SearchResultsWrapper<'a>>> {
    request: RequestBuilder<'a, T>,
    body: T,
}

impl<'a, T: From<SearchResultsWrapper<'a>>> Deref for Response<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.body
    }
}

impl<'a, T: From<SearchResultsWrapper<'a>>> RequestBuilder<'a, T> {
    pub fn page(&mut self, page_num: i32) -> &Self {
        self.inner.page = page_num;
        self
    }

    // TODO: Handle errors
    pub fn execute(self) -> Response<'a, T> {
        let json = serde_json::to_string(&self.inner).unwrap();

        let result = self.http
            .post(protocol::SEARCH_URL)
            .body(json)
            .send()
            .unwrap()
            .json()
            .unwrap();

        Response {
            request: self,
            body: T::from(result),
        }
    }
}

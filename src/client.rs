extern crate serde_json;
extern crate reqwest;

use models::*;
use protocol::{categories, SearchRequest, SearchResult, SearchResultsWrapper};
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

    pub fn songs_by_artist_id(&self, id: &'a str) -> RequestBuilder<Song> {
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

    pub fn songs_by_title(&self, title: &'a str, match_type: MatchType) -> RequestBuilder<Song> {
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
                           -> RequestBuilder<Song> {
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

    pub fn artists_by_name(&self, name: &'a str, match_type: MatchType) -> RequestBuilder<Artist> {
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
pub struct RequestBuilder<'a, T> {
    http: Arc<reqwest::Client>,
    inner: SearchRequest<'a>,
    response_type: PhantomData<T>,
}

impl<'a, T> RequestBuilder<'a, T> {
    pub fn page(&self, page_num: i32) -> Self {
        RequestBuilder {
            http: self.http.clone(),
            response_type: self.response_type,
            inner: SearchRequest { page: page_num, ..self.inner },
        }
    }
}

impl<'a, T> RequestBuilder<'a, T>
    where T: From<SearchResult<'a>>
{
    // TODO: Handle errors
    pub fn execute(self) -> Response<'a, T> {
        let json = serde_json::to_string(&self.inner).unwrap();

        let result: SearchResultsWrapper = self.http
            .post(protocol::SEARCH_URL)
            .body(json)
            .send()
            .unwrap()
            .json()
            .unwrap();

        Response {
            request: self,
            body: Paginated::from(result),
        }
    }
}

#[derive(Debug)]
pub struct Response<'a, T> {
    request: RequestBuilder<'a, T>,
    body: Paginated<T>,
}

impl<'a, T> Response<'a, T> {
    pub fn prev_page(&self) -> Option<RequestBuilder<'a, T>> {
        self.change_page(-1)
    }

    pub fn next_page(&self) -> Option<RequestBuilder<'a, T>> {
        self.change_page(1)
    }

    fn change_page(&self, delta: i32) -> Option<RequestBuilder<'a, T>> {
        let page = self.request.inner.page;
        let next_page = page + delta;

        if next_page <= 0 || next_page >= self.body.total_pages {
            None
        } else {
            Some(self.request.page(next_page))
        }
    }
}

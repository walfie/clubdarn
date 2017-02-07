extern crate serde_json;
extern crate reqwest;

use model::*;
use category;
use protocol::search;
use std::marker::PhantomData;
use std::sync::Arc;
use std::ops::Deref;

pub struct Client<'a> {
    http: Arc<reqwest::Client>,
    default_request: search::Request<'a>,
}

impl<'a> Client<'a> {
    pub fn new(app_ver: &'a str, device_id: &'a str, device_nm: &'a str, os_ver: &'a str) -> Self {
        let req = search::Request {
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
            inner: search::Request {
                artist_id: Some(id),
                category_cd: category::ARTIST_NAME.0,
                ..self.default_request
            },
        }
    }

    pub fn songs_by_title(&self, title: &'a str, match_type: MatchType) -> RequestBuilder<Song> {
        RequestBuilder {
            http: self.http.clone(),
            response_type: PhantomData,
            inner: search::Request {
                song_name: Some(title),
                category_cd: category::SONG_NAME.0,
                song_match_type: Some(match_type.0),
                ..self.default_request
            },
        }
    }

    pub fn songs_by_series(&self,
                           title: &'a str,
                           category: category::CategoryId)
                           -> RequestBuilder<Song> {
        RequestBuilder {
            http: self.http.clone(),
            response_type: PhantomData,
            inner: search::Request {
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
            inner: search::Request {
                artist_name: Some(name),
                category_cd: category::ARTIST_NAME.0,
                artist_match_type: Some(match_type.0),
                ..self.default_request
            },
        }
    }

    pub fn series_by_category(&self, category: category::CategoryId) -> RequestBuilder<Series> {
        RequestBuilder {
            http: self.http.clone(),
            response_type: PhantomData,
            inner: search::Request { category_cd: category.0, ..self.default_request },
        }
    }

    pub fn new_songs_by_category(&self, category: category::CategoryId) -> RequestBuilder<Song> {
        RequestBuilder {
            http: self.http.clone(),
            response_type: PhantomData,
            inner: search::Request { category_cd: category.0, ..self.default_request },
        }
    }
}

pub struct MatchType(pub &'static str);
pub const STARTS_WITH: MatchType = MatchType("0");
pub const CONTAINS: MatchType = MatchType("1");

#[derive(Debug)]
pub struct RequestBuilder<'a, T> {
    http: Arc<reqwest::Client>,
    inner: search::Request<'a>,
    response_type: PhantomData<T>,
}

impl<'a, T> RequestBuilder<'a, T> {
    pub fn page(&self, page_num: i32) -> Self {
        RequestBuilder {
            http: self.http.clone(),
            response_type: self.response_type,
            inner: search::Request { page: page_num, ..self.inner },
        }
    }
}

impl<'a, T> RequestBuilder<'a, T>
    where T: SearchModel<'a>
{
    // TODO: Handle errors
    pub fn execute(self) -> Response<'a, T> {
        let json = serde_json::to_string(&self.inner).unwrap();

        let result: search::Response = self.http
            .post(search::API_URL)
            .body(json)
            .send()
            .unwrap()
            .json()
            .unwrap();

        let body =
            Paginated::from_search_response(self.inner.page, self.inner.category_cd.into(), result);

        Response {
            request: self,
            body: body,
        }
    }
}

#[derive(Debug)]
pub struct Response<'a, T> {
    pub request: RequestBuilder<'a, T>,
    pub body: Paginated<'a, T>,
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

        if next_page > 0 && next_page <= self.body.total_pages {
            Some(self.request.page(next_page))
        } else {
            None
        }
    }
}

impl<'a, T> Deref for Response<'a, T> {
    type Target = Paginated<'a, T>;

    fn deref(&self) -> &Paginated<'a, T> {
        &self.body
    }
}

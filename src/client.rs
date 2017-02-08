extern crate serde_json;
extern crate reqwest;

use model::*;
use category;
use protocol::api;
use protocol::search;
use std::marker::PhantomData;
use std::sync::Arc;

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
        req.song_match_type = Some(match_type.0);

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
        req.artist_match_type = Some(match_type.0);

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
}

pub struct MatchType(pub &'static str);
pub const STARTS_WITH: MatchType = MatchType("0");
pub const CONTAINS: MatchType = MatchType("1");

pub struct RequestBuilder<'a, RequestT, ResponseItemT: 'a> {
    http: Arc<reqwest::Client>,
    request: RequestT,
    response_item_type: PhantomData<&'a ResponseItemT>,
}

pub struct ResponseWrapper<'a, R: 'a, I: 'a> {
    request: RequestBuilder<'a, R, I>,
    pub body: Paginated<'a, I>,
}

impl<'a, R, I> RequestBuilder<'a, R, I>
    where R: api::Request<'a>
{
    pub fn page(&self, page_num: i32) -> Self {
        RequestBuilder {
            http: self.http.clone(),
            request: self.request.page(page_num),
            response_item_type: self.response_item_type,
        }
    }
}

impl<'a, R, I> RequestBuilder<'a, R, I>
    where R: api::Request<'a>,
          I: From<<R::ResponseType as api::Response<'a>>::ItemType>
{
    pub fn send(self) -> ResponseWrapper<'a, R, I> {
        use protocol::api::Response;

        let json = serde_json::to_string(&self.request).unwrap();

        let response: R::ResponseType = self.http
            .post(R::url())
            .body(json)
            .send()
            .unwrap()
            .json()
            .unwrap();

        let body = Paginated {
            page: self.request.get_page(),
            category_id: self.request.category(),
            total_items: response.total_items(),
            total_pages: response.total_pages(),
            items: response.items().into_iter().map(I::from).collect(),
        };

        ResponseWrapper {
            request: self,
            body: body,
        }
    }
}

impl<'a, R, I> ResponseWrapper<'a, R, I>
    where R: api::Request<'a>
{
    pub fn prev_page(&self) -> Option<RequestBuilder<'a, R, I>> {
        self.change_page(-1)
    }

    pub fn next_page(&self) -> Option<RequestBuilder<'a, R, I>> {
        self.change_page(1)
    }

    fn change_page(&self, delta: i32) -> Option<RequestBuilder<'a, R, I>> {
        let page = self.request.request.get_page();
        let next_page = page + delta;

        if next_page > 0 && next_page <= self.body.total_pages {
            Some(self.request.page(next_page))
        } else {
            None
        }
    }
}

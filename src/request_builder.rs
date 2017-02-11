extern crate reqwest;
extern crate serde_json;

use std::marker::PhantomData;
use std::sync::Arc;

use category;
use client;
use client::MatchType;
use model::*;
use protocol::{api, exist, recommend, search};

#[must_use = "RequestBuilder does nothing until you call `send`"]
pub struct RequestBuilder<'a, RequestT, ResponseItemT: 'a> {
    pub http: Arc<reqwest::Client>,
    pub request: RequestT,
    pub response_item_type: PhantomData<&'a ResponseItemT>,
}

impl<'a, I> RequestBuilder<'a, &'a client::Metadata<'a>, I> {
    fn default_request<R>(self) -> RequestBuilder<'a, R, I>
        where R: api::Request<'a>
    {
        RequestBuilder {
            http: self.http,
            request: R::from_client_metadata(self.request),
            response_item_type: PhantomData,
        }
    }
}

impl<'a> RequestBuilder<'a, &'a client::Metadata<'a>, Song<'a>> {
    pub fn by_title(self,
                    title: &'a str,
                    match_type: MatchType)
                    -> RequestBuilder<search::Request, Song> {
        let mut req = self.default_request::<search::Request>();
        req.request.song_name = Some(title);
        req.request.song_match_type = Some(match_type.into());
        req.request.category_cd = category::SONG_NAME.0;
        req
    }

    pub fn starting_with(self, title: &'a str) -> RequestBuilder<search::Request, Song> {
        self.by_title(title, MatchType::StartsWith)
    }

    pub fn containing(self, title: &'a str) -> RequestBuilder<search::Request, Song> {
        self.by_title(title, MatchType::Contains)
    }

    pub fn by_artist_id(self, id: i32) -> RequestBuilder<'a, search::Request<'a>, Song<'a>> {
        let mut req = self.default_request::<search::Request>();
        req.request.artist_id = Some(id);
        req.request.category_cd = ::category::ARTIST_NAME.0;
        req
    }

    pub fn by_series(self,
                     title: &'a str,
                     category_id: &'a str)
                     -> RequestBuilder<'a, search::Request<'a>, Song<'a>> {
        let mut req = self.default_request::<search::Request>();
        req.request.program_title = Some(title);
        req.request.category_cd = category_id;
        req
    }

    pub fn recent(self, category_id: &'a str) -> RequestBuilder<search::Request, Song> {
        let mut req = self.default_request::<search::Request>();
        req.request.category_cd = category_id;
        req
    }

    pub fn by_ids(self, ids: Vec<i32>) -> RequestBuilder<'a, exist::Request<'a>, Song<'a>> {
        let mut req = self.default_request::<exist::Request>();
        req.request.is_exist = ids.iter().map(|id| exist::RequestItem::from_id(*id)).collect();
        req
    }

    pub fn by_title_and_artist(self,
                               titles_and_artists: Vec<client::TitleAndArtist<'a>>)
                               -> RequestBuilder<exist::Request, Song> {
        let mut req = self.default_request::<exist::Request>();
        req.request.is_exist = titles_and_artists.iter()
            .map(|x| exist::RequestItem::from_title_and_artist(x.title, x.artist))
            .collect();

        req
    }

    pub fn similar_to(self, song_id: i32) -> RequestBuilder<'a, recommend::Request<'a>, Song<'a>> {
        let mut req = self.default_request::<recommend::Request>();
        let mut song_id_str = song_id.to_string();
        if (song_id_str.len() as i32) > 4 {
            song_id_str.insert(4, '-');
        }

        req.request.request_no_list = song_id_str.into();
        req
    }
}

impl<'a> RequestBuilder<'a, &'a client::Metadata<'a>, Artist<'a>> {
    pub fn by_name(self,
                   name: &'a str,
                   match_type: MatchType)
                   -> RequestBuilder<search::Request, Artist> {
        let mut req = self.default_request::<search::Request>();
        req.request.artist_name = Some(name);
        req.request.artist_match_type = Some(match_type.into());
        req.request.category_cd = category::ARTIST_NAME.0;
        req
    }

    pub fn starting_with(self, name: &'a str) -> RequestBuilder<search::Request, Artist> {
        self.by_name(name, MatchType::StartsWith)
    }

    pub fn containing(self, name: &'a str) -> RequestBuilder<search::Request, Artist> {
        self.by_name(name, MatchType::Contains)
    }
}

impl<'a> RequestBuilder<'a, &'a client::Metadata<'a>, Series<'a>> {
    pub fn by_category(self, category_id: &'a str) -> RequestBuilder<search::Request, Series> {
        let mut req = self.default_request::<search::Request>();
        req.request.category_cd = category_id;
        req
    }
}

impl<'a, R, I> RequestBuilder<'a, R, I>
    where R: api::Request<'a>
{
    pub fn set_page(&mut self, page_num: i32) -> &Self {
        self.request.set_page(page_num);
        self
    }

    pub fn set_serial_no(&mut self, serial_no: &'a str) -> &Self {
        self.request.set_serial_no(serial_no);
        self
    }
}

impl<'a, R, I> RequestBuilder<'a, R, I>
    where R: api::Request<'a>,
          I: From<<R::ResponseType as api::Response<'a>>::ItemType>
{
    pub fn send(&self) -> Paginated<'a, I> {
        use protocol::api::Response;

        let request = self.http.post(R::url());

        let request_body = match R::request_type() {
            api::RequestType::Json => request.json(&self.request),
            api::RequestType::FormData => request.form(&self.request),
        };

        // TODO: Error handling
        let response: R::ResponseType = request_body
            .send()
            .unwrap()
            .json()
            .unwrap();

        // Doing this weird thing because `items()` consumes `response`
        let total_items = response.total_items();
        let mut body = Paginated {
            page: self.request.get_page(),
            category_id: self.request.category(),
            total_items: 0,
            total_pages: response.total_pages(),
            items: response.items().into_iter().map(I::from).collect(),
        };

        body.total_items = total_items.unwrap_or(body.items.len() as i32);

        body
    }
}

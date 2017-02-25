extern crate serde_json;
extern crate reqwest;

use category;
use category::*;
use error::*;
use model::*;
use protocol::{api, exist, recommend, search};
use std::borrow::Cow;
use std::marker::PhantomData;
use std::sync::Arc;

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

impl<'a> Default for Metadata<'a> {
    fn default() -> Self {
        Metadata {
            app_ver: "1.2.0", // Denmoku Mini app version
            device_id: "",
            device_nm: env!("CARGO_PKG_NAME"),
            os_ver: env!("CARGO_PKG_VERSION"),
            serial_no: None,
        }
    }
}

pub struct Pending<'a>(&'a Metadata<'a>);

#[derive(Debug, Serialize)]
pub struct TitleAndArtist<'a> {
    pub title: Cow<'a, str>,
    pub artist: Cow<'a, str>,
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


impl<'a> Client<'a> {
    pub fn new(meta: Metadata<'a>) -> Result<Self> {
        let http = reqwest::Client::new()?;

        Ok(Client {
            http: Arc::new(http),
            meta: meta,
        })
    }

    pub fn default() -> Result<Self> {
        Self::new(Default::default())
    }

    fn request_builder<T, U>(&self, req: T) -> RequestBuilder<T, U> {
        RequestBuilder {
            http: self.http.clone(),
            request: req,
            response_item_type: PhantomData,
        }
    }

    pub fn set_default_serial_no(mut self, serial_no: Option<&'a str>) -> Self {
        self.meta.serial_no = serial_no;
        self
    }

    pub fn artists(&self) -> RequestBuilder<Pending, Artist> {
        self.request_builder(Pending(&self.meta))
    }

    pub fn songs(&self) -> RequestBuilder<Pending, Song> {
        self.request_builder(Pending(&self.meta))
    }

    pub fn series(&self) -> RequestBuilder<Pending, Series> {
        self.request_builder(Pending(&self.meta))
    }
}

#[must_use = "RequestBuilder does nothing until you call `send`"]
pub struct RequestBuilder<RequestT, ResponseItemT> {
    http: Arc<reqwest::Client>,
    request: RequestT,
    response_item_type: PhantomData<ResponseItemT>,
}

impl<'a, I> RequestBuilder<Pending<'a>, I> {
    fn default_request<R>(&self) -> RequestBuilder<R, I>
        where R: api::Request<'a>
    {
        RequestBuilder {
            http: self.http.clone(),
            request: R::from_client_metadata(self.request.0),
            response_item_type: PhantomData,
        }
    }
}

impl<'a> RequestBuilder<Pending<'a>, Song> {
    // TODO: Require that strings are non-empty
    pub fn by_title(self,
                    title: &'a str,
                    match_type: MatchType)
                    -> RequestBuilder<search::Request, Song> {
        let mut req = self.default_request::<search::Request>();
        req.request.song_name = Some(title);
        req.request.song_match_type = Some(match_type.into());
        req.request.category_cd = category::SONG_NAME.id.0;
        req
    }

    pub fn starting_with(self, title: &'a str) -> RequestBuilder<search::Request, Song> {
        self.by_title(title, MatchType::StartsWith)
    }

    pub fn containing(self, title: &'a str) -> RequestBuilder<search::Request, Song> {
        self.by_title(title, MatchType::Contains)
    }

    pub fn by_artist_id(self, id: i32) -> RequestBuilder<search::Request<'a>, Song> {
        self.by_artist_in_category_id(id, category::ARTIST_NAME.id.0)
    }

    pub fn by_artist_in_category_id(self,
                                    artist_id: i32,
                                    category_id: &'a str)
                                    -> RequestBuilder<search::Request, Song> {
        let mut req = self.default_request::<search::Request>();
        req.request.artist_id = Some(artist_id);
        req.request.category_cd = category_id;
        req
    }

    pub fn by_series_in_category_id(self,
                                    title: &'a str,
                                    category_id: &'a str)
                                    -> RequestBuilder<search::Request<'a>, Song> {
        let mut req = self.default_request::<search::Request>();
        req.request.program_title = Some(title);
        req.request.category_cd = category_id;
        req
    }

    pub fn by_series(self,
                     title: &'a str,
                     category: Category<SeriesCategory>)
                     -> RequestBuilder<search::Request, Song> {
        self.by_series_in_category_id(title, category.id.0)
    }

    pub fn by_category_id(self, category_id: &'a str) -> RequestBuilder<search::Request, Song> {
        let mut req = self.default_request::<search::Request>();
        req.request.category_cd = category_id;
        req
    }

    pub fn by_category(self,
                       category: ::category::Category<SongCategory>)
                       -> RequestBuilder<search::Request<'a>, Song> {
        self.by_category_id(category.id.0)
    }

    pub fn by_ids(self, ids: &[i32]) -> RequestBuilder<exist::Request<'a>, Song> {
        let mut req = self.default_request::<exist::Request>();
        req.request.is_exist = ids.iter().map(|id| exist::RequestItem::from_id(*id)).collect();
        req
    }

    pub fn by_id(self, id: i32) -> RequestBuilder<exist::Request<'a>, Song> {
        self.by_ids(&[id])
    }

    pub fn by_titles_and_artists(self,
                                 titles_and_artists: &'a [TitleAndArtist<'a>])
                                 -> RequestBuilder<exist::Request<'a>, Song> {
        let mut req = self.default_request::<exist::Request>();
        req.request.is_exist = titles_and_artists.iter()
            .map(|x| exist::RequestItem::from_title_and_artist(&x.title, &x.artist))
            .collect();

        req
    }

    pub fn by_title_and_artist(self,
                               title: &'a str,
                               artist: &'a str)
                               -> RequestBuilder<exist::Request<'a>, Song> {
        let item = exist::RequestItem::from_title_and_artist(title, artist);

        let mut req = self.default_request::<exist::Request>();
        req.request.is_exist = vec![item];
        req
    }

    pub fn similar_to(self, song_id: i32) -> RequestBuilder<recommend::Request<'a>, Song> {
        let mut req = self.default_request::<recommend::Request>();
        let mut song_id_str = song_id.to_string();

        // The recommend API requires song IDs to be in the format "1234-56"
        if (song_id_str.len() as i32) > 4 {
            song_id_str.insert(4, '-');
        }

        req.request.request_no_list = song_id_str.into();
        req
    }
}

impl<'a> RequestBuilder<Pending<'a>, Artist> {
    pub fn by_name(self,
                   name: &'a str,
                   match_type: MatchType)
                   -> RequestBuilder<search::Request, Artist> {
        let mut req = self.default_request::<search::Request>();
        req.request.artist_name = Some(name);
        req.request.artist_match_type = Some(match_type.into());
        req.request.category_cd = category::ARTIST_NAME.id.0;
        req
    }

    pub fn starting_with(self, name: &'a str) -> RequestBuilder<search::Request, Artist> {
        self.by_name(name, MatchType::StartsWith)
    }

    pub fn containing(self, name: &'a str) -> RequestBuilder<search::Request, Artist> {
        self.by_name(name, MatchType::Contains)
    }

    pub fn live_performance(self) -> RequestBuilder<search::Request<'a>, Artist> {
        let mut req = self.default_request::<search::Request>();
        req.request.category_cd = category::LIVE_PERFORMANCE.id.0;
        req
    }
}

impl<'a> RequestBuilder<Pending<'a>, Series> {
    pub fn by_category_id(self,
                          category_id: &'a str)
                          -> RequestBuilder<search::Request<'a>, Series> {
        let mut req = self.default_request::<search::Request>();
        req.request.category_cd = category_id;
        req
    }

    pub fn by_category(self,
                       category: Category<SeriesCategory>)
                       -> RequestBuilder<search::Request<'a>, Series> {
        self.by_category_id(category.id.0)
    }
}

impl<'a, R, I> RequestBuilder<R, I>
    where R: api::Request<'a>
{
    pub fn set_page(&mut self, page_num: i32) -> &Self {
        self.request.set_page(page_num);
        self
    }

    pub fn set_serial_no(&mut self, serial_no: Option<&'a str>) -> &Self {
        self.request.set_serial_no(serial_no);
        self
    }
}

impl<'a, R, I> RequestBuilder<R, I>
    where R: api::Request<'a>,
          I: From<<R::ResponseType as api::Response>::ItemType>
{
    pub fn send(&'a self) -> Result<Paginated<I>> {
        use protocol::api::Response;

        let request = self.http.post(R::url());

        let request_body = match R::request_type() {
            api::RequestType::Json => request.json(&self.request),
            api::RequestType::FormData => request.form(&self.request),
        };

        // TODO: Use enum errors
        let response: R::ResponseType = request_body.send()
            .chain_err(|| "failed to send request")?
            .json()
            .chain_err(|| "failed to parse JSON response")?;

        let artist_category_id = self.request
            .category()
            .map_or(category::ARTIST_NAME.id.0,
                    |c| category::artist_category(c).id.0)
            .into();

        let series_category_id = self.request
            .category()
            .and_then(category::series_category)
            .map(|c| c.id.0.into());

        // Doing this weird `total_items: 0` thing because `take_items()` consumes `response`
        let total_items = response.total_items();
        let mut body = Paginated {
            page: self.request.page(),
            artist_category_id: artist_category_id,
            series_category_id: series_category_id,
            total_items: 0,
            total_pages: response.total_pages(),
            items: response.take_items().into_iter().map(I::from).collect(),
        };

        body.total_items = total_items.unwrap_or(body.items.len() as i32);

        Ok(body)
    }
}

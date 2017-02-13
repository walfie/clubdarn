extern crate serde_json;
extern crate reqwest;

use std::borrow::Cow;
use std::marker::PhantomData;
use std::sync::Arc;

use category;
use category::*;
use model::*;
use protocol::{api, exist, recommend, search};

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

pub struct Pending<'a>(&'a Metadata<'a>);

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

    fn request_builder<T, U>(&self, req: T) -> RequestBuilder<T, U> {
        RequestBuilder {
            http: self.http.clone(),
            request: req,
            response_item_type: PhantomData,
        }
    }

    pub fn default_serial_no(mut self, serial_no: Option<&'a str>) -> Self {
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
    pub fn by_title(&self,
                    title: &'a str,
                    match_type: MatchType)
                    -> RequestBuilder<search::Request, Song> {
        let mut req = self.default_request::<search::Request>();
        req.request.song_name = Some(title);
        req.request.song_match_type = Some(match_type.into());
        req.request.category_cd = category::SONG_NAME.id.0;
        req
    }

    pub fn starting_with(&self, title: &'a str) -> RequestBuilder<search::Request, Song> {
        self.by_title(title, MatchType::StartsWith)
    }

    pub fn containing(&self, title: &'a str) -> RequestBuilder<search::Request, Song> {
        self.by_title(title, MatchType::Contains)
    }

    pub fn by_artist_id(&self, id: i32) -> RequestBuilder<search::Request, Song> {
        self.by_artist_in_category_id(id, category::ARTIST_NAME.id.0)
    }

    pub fn by_artist_in_category_id(&self,
                                    artist_id: i32,
                                    category_id: Cow<'a, str>)
                                    -> RequestBuilder<search::Request, Song> {
        let mut req = self.default_request::<search::Request>();
        req.request.artist_id = Some(artist_id);
        req.request.category_cd = category_id;
        req
    }

    pub fn by_series_in_category_id<T>(&self,
                                       title: &'a str,
                                       category_id: T)
                                       -> RequestBuilder<search::Request, Song>
        where T: Into<Cow<'a, str>>
    {
        let mut req = self.default_request::<search::Request>();
        req.request.program_title = Some(title);
        req.request.category_cd = category_id.into();
        req
    }

    pub fn by_series<T>(&self,
                        title: &'a str,
                        category: Category<SeriesCategory>)
                        -> RequestBuilder<search::Request, Song> {
        self.by_series_in_category_id(title, category.id.0)
    }

    pub fn by_category_id<T>(&self, category_id: T) -> RequestBuilder<search::Request, Song>
        where T: Into<Cow<'a, str>>
    {
        let mut req = self.default_request::<search::Request>();
        req.request.category_cd = category_id.into();
        req
    }

    pub fn by_category(&self,
                       category: ::category::Category<SongCategory>)
                       -> RequestBuilder<search::Request, Song> {
        self.by_category_id(category.id.0)
    }

    pub fn by_ids(&self, ids: Vec<i32>) -> RequestBuilder<exist::Request, Song> {
        let mut req = self.default_request::<exist::Request>();
        req.request.is_exist = ids.iter().map(|id| exist::RequestItem::from_id(*id)).collect();
        req
    }

    pub fn by_id(&self, id: i32) -> RequestBuilder<exist::Request, Song> {
        self.by_ids(vec![id])
    }

    pub fn by_titles_and_artists(&self,
                                 titles_and_artists: Vec<TitleAndArtist<'a>>)
                                 -> RequestBuilder<exist::Request, Song> {
        let mut req = self.default_request::<exist::Request>();
        req.request.is_exist = titles_and_artists.iter()
            .map(|x| exist::RequestItem::from_title_and_artist(x.title, x.artist))
            .collect();

        req
    }

    pub fn by_title_and_artist(&self,
                               title: &'a str,
                               artist: &'a str)
                               -> RequestBuilder<exist::Request, Song> {
        let info = TitleAndArtist {
            title: title,
            artist: artist,
        };
        self.by_titles_and_artists(vec![info])
    }

    pub fn similar_to(&self, song_id: i32) -> RequestBuilder<recommend::Request, Song> {
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
    pub fn by_name(&self,
                   name: &'a str,
                   match_type: MatchType)
                   -> RequestBuilder<search::Request, Artist> {
        let mut req = self.default_request::<search::Request>();
        req.request.artist_name = Some(name);
        req.request.artist_match_type = Some(match_type.into());
        req.request.category_cd = category::ARTIST_NAME.id.0;
        req
    }

    pub fn starting_with(&self, name: &'a str) -> RequestBuilder<search::Request, Artist> {
        self.by_name(name, MatchType::StartsWith)
    }

    pub fn containing(&self, name: &'a str) -> RequestBuilder<search::Request, Artist> {
        self.by_name(name, MatchType::Contains)
    }
}

impl<'a> RequestBuilder<Pending<'a>, Series> {
    pub fn by_category_id<T>(&self, category_id: T) -> RequestBuilder<search::Request, Series>
        where T: Into<Cow<'a, str>>
    {
        let mut req = self.default_request::<search::Request>();
        req.request.category_cd = category_id.into();
        req
    }

    pub fn by_category<T>(&self,
                          category: Category<SeriesCategory>)
                          -> RequestBuilder<search::Request, Series> {
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

    pub fn set_serial_no(&mut self, serial_no: &'a str) -> &Self {
        self.request.set_serial_no(serial_no);
        self
    }
}

impl<'a, R, I> RequestBuilder<R, I>
    where R: api::Request<'a>,
          I: From<<R::ResponseType as api::Response>::ItemType>
{
    pub fn send(&self) -> Paginated<I> {
        use protocol::api::Response;

        let request = self.http.post(R::url());

        let request_body = match R::request_type() {
            api::RequestType::Json => request.json(&self.request),
            api::RequestType::FormData => request.form(&self.request),
        };

        // TODO: Error handling
        let response: R::ResponseType = request_body.send()
            .unwrap()
            .json()
            .unwrap();

        let artist_category_id = self.request
            .category()
            .map_or(category::ARTIST_NAME.id.0,
                    |c| category::artist_category(c).id.0)
            .into();

        let series_category_id = self.request
            .category()
            .and_then(category::series_category)
            .map(|c| c.id.0.into());

        // Doing this weird `total_items: 0` thing because `items()` consumes `response`
        let total_items = response.total_items();
        let mut body = Paginated {
            page: self.request.get_page(),
            artist_category_id: artist_category_id,
            series_category_id: series_category_id,
            total_items: 0,
            total_pages: response.total_pages(),
            items: response.items().into_iter().map(I::from).collect(),
        };

        body.total_items = total_items.unwrap_or(body.items.len() as i32);

        body
    }
}

use client::ClientMetadata;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub trait Request<'a>: Serialize {
    type ResponseType: Response<'a>;

    fn url() -> &'a str;
    fn from_client_metadata(meta: &ClientMetadata<'a>) -> Self;
    fn get_page(&self) -> i32;
    fn page(&self, page_num: i32) -> Self;
    fn category(&self) -> Cow<'a, str>;
}

pub trait Response<'a>: Deserialize {
    type ItemType;

    fn items(self) -> Vec<Self::ItemType>;
    fn total_pages(&self) -> i32;
    fn total_items(&self) -> i32;
}

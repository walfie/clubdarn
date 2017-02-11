use client::ClientMetadata;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub enum RequestType {
    Json,
    FormData,
}

pub trait Request<'a>: Serialize {
    type ResponseType: Response<'a>;

    fn request_type() -> RequestType;

    fn url() -> &'a str;
    fn from_client_metadata(meta: &ClientMetadata<'a>) -> Self;

    fn set_serial_no(&mut self, serial_no: &'a str) -> &Self;
    fn unset_serial_no(&mut self) -> &Self;

    fn get_page(&self) -> i32;
    fn set_page(&mut self, page_num: i32) -> &Self;

    fn category(&self) -> Option<Cow<'a, str>>;
}

pub trait Response<'a>: Deserialize {
    type ItemType;

    // TODO: Maybe make this not consume self
    fn items(self) -> Vec<Self::ItemType>;
    fn total_pages(&self) -> i32;
    fn total_items(&self) -> Option<i32>;
}

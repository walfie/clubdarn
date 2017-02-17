use client;
use serde::{Deserialize, Serialize};

pub enum RequestType {
    Json,
    FormData,
}

pub trait Request<'a>: Serialize {
    type ResponseType: Response;

    fn request_type() -> RequestType;

    fn url() -> &'a str;
    fn from_client_metadata(meta: &client::Metadata<'a>) -> Self;

    fn set_serial_no(&mut self, serial_no: Option<&'a str>) -> &Self;

    fn page(&self) -> i32;
    fn set_page(&mut self, page_num: i32) -> &Self;

    fn category(&'a self) -> Option<&'a str>;
}

pub trait Response: Deserialize {
    type ItemType;

    fn take_items(self) -> Vec<Self::ItemType>;
    fn total_pages(&self) -> i32;
    fn total_items(&self) -> Option<i32>;
}

use client::ClientMetadata;
use serde::{Deserialize, Serialize};

pub trait Request<'a>: Serialize {
    type ResponseType: Response;

    fn url() -> &'a str;
    fn from_client_metadata(meta: &ClientMetadata<'a>) -> Self;
}

pub trait Response: Deserialize {
    type ItemType: Item;
}

pub trait Item: Deserialize {}

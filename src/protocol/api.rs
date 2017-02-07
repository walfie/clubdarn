use serde::{Deserialize, Serialize};

pub trait Request<'a>: Serialize {
    type ResponseType: Response;

    fn url() -> &'a str;
}

pub trait Response: Deserialize {
    type ItemType: Item;
}

pub trait Item: Deserialize {}

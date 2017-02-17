use reqwest;
use serde_json;

error_chain! {
    foreign_links {
        Http(reqwest::Error);
        Json(serde_json::Error);
    }
}

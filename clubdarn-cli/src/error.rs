use clap;
use clubdarn;
use serde_json;

error_chain! {
    links {
        Client(clubdarn::Error, clubdarn::error::ErrorKind);
    }

    foreign_links {
        Json(serde_json::Error);
        Input(clap::Error);
    }
}

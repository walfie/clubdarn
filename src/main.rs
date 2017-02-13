extern crate clubdarn;
extern crate serde;
extern crate serde_json;

use serde::Serialize;

use clubdarn::*;

fn pretty<T: Serialize>(t: &T) -> String {
    serde_json::to_string_pretty(t).unwrap()
}

fn main() {
    let client = Client::new("1.2.0", "test", "hello", "4.4.4").default_serial_no(Some("AB316238"));

    let songs = client.songs().similar_to(372915).send();
    println!("{}", pretty(&songs));
}

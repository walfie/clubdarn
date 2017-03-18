use rocket::http::hyper::header::{CacheControl, CacheDirective};
use rocket::response::{self, Responder, Response};

// Simplified version of:
// https://github.com/SergioBenitez/Rocket/issues/25#issuecomment-271065434
pub struct Cors<R>(pub R);

impl<'a, R> Responder<'a> for Cors<R>
    where R: Responder<'a>
{
    fn respond(self) -> response::Result<'a> {
        let response = Response::build_from(self.0.respond()?)
            .raw_header("Access-Control-Allow-Origin", "*")
            .raw_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
            .raw_header("Access-Control-Allow-Headers", "Content-Type")
            .finalize();

        Ok(response)
    }
}

pub struct Cached<R> {
    pub inner: R,
    pub max_age_seconds: u32,
}


impl<'a, R> Responder<'a> for Cached<R>
    where R: Responder<'a>
{
    fn respond(self) -> response::Result<'a> {
        let header = CacheControl(vec![CacheDirective::MaxAge(self.max_age_seconds)]);
        let response = Response::build_from(self.inner.respond()?)
            .header(header)
            .finalize();

        Ok(response)
    }
}

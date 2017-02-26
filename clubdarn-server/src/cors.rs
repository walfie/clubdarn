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
            .finalize();

        Ok(response)
    }
}

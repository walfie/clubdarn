extern crate reqwest;
extern crate serde_json;

use std::marker::PhantomData;
use std::sync::Arc;

use model::*;
use protocol::api;

#[must_use = "RequestBuilder does nothing until you call `send`"]
pub struct RequestBuilder<'a, RequestT, ResponseItemT: 'a> {
    pub http: Arc<reqwest::Client>,
    pub request: RequestT,
    pub response_item_type: PhantomData<&'a ResponseItemT>,
}

impl<'a, R, I> RequestBuilder<'a, R, I>
    where R: api::Request<'a>
{
    pub fn set_page(&mut self, page_num: i32) -> &Self {
        self.request.set_page(page_num);
        self
    }

    pub fn set_serial_no(&mut self, serial_no: &'a str) -> &Self {
        self.request.set_serial_no(serial_no);
        self
    }
}

impl<'a, R, I> RequestBuilder<'a, R, I>
    where R: api::Request<'a>,
          I: From<<R::ResponseType as api::Response<'a>>::ItemType>
{
    pub fn send(&self) -> Paginated<'a, I> {
        use protocol::api::Response;

        let request = self.http.post(R::url());

        let request_body = match R::request_type() {
            api::RequestType::Json => request.json(&self.request),
            api::RequestType::FormData => request.form(&self.request),
        };

        // TODO: Error handling
        let response: R::ResponseType = request_body
            .send()
            .unwrap()
            .json()
            .unwrap();

        // Doing this weird thing because `items()` consumes `response`
        let total_items = response.total_items();
        let mut body = Paginated {
            page: self.request.get_page(),
            category_id: self.request.category(),
            total_items: 0,
            total_pages: response.total_pages(),
            items: response.items().into_iter().map(I::from).collect(),
        };

        body.total_items = total_items.unwrap_or(body.items.len() as i32);

        body
    }
}
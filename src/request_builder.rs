extern crate reqwest;
extern crate serde_json;

use std::sync::Arc;
use protocol::api;
use model::*;
use std::marker::PhantomData;

pub struct RequestBuilder<'a, RequestT, ResponseItemT: 'a> {
    pub http: Arc<reqwest::Client>,
    pub request: RequestT,
    pub response_item_type: PhantomData<&'a ResponseItemT>,
}

pub struct ResponseWrapper<'a, R: 'a, I: 'a> {
    request: RequestBuilder<'a, R, I>,
    pub body: Paginated<'a, I>,
}

impl<'a, R, I> RequestBuilder<'a, R, I>
    where R: api::Request<'a>
{
    pub fn page(&self, page_num: i32) -> Self {
        RequestBuilder {
            http: self.http.clone(),
            request: self.request.page(page_num),
            response_item_type: self.response_item_type,
        }
    }
}

impl<'a, R, I> RequestBuilder<'a, R, I>
    where R: api::Request<'a>,
          I: From<<R::ResponseType as api::Response<'a>>::ItemType>
{
    pub fn send(self) -> ResponseWrapper<'a, R, I> {
        use protocol::api::Response;

        let json = serde_json::to_string(&self.request).unwrap();

        let response: R::ResponseType = self.http
            .post(R::url())
            .body(json)
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

        ResponseWrapper {
            request: self,
            body: body,
        }
    }
}

impl<'a, R, I> ResponseWrapper<'a, R, I>
    where R: api::Request<'a>
{
    pub fn prev_page(&self) -> Option<RequestBuilder<'a, R, I>> {
        self.change_page(-1)
    }

    pub fn next_page(&self) -> Option<RequestBuilder<'a, R, I>> {
        self.change_page(1)
    }

    fn change_page(&self, delta: i32) -> Option<RequestBuilder<'a, R, I>> {
        let page = self.request.request.get_page();
        let next_page = page + delta;

        if next_page > 0 && next_page <= self.body.total_pages {
            Some(self.request.page(next_page))
        } else {
            None
        }
    }
}

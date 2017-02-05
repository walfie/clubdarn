use protocol::{SearchResult, SearchResultsWrapper};
use std::borrow::Cow;
use std::convert::From;

#[derive(Debug)]
pub struct Paginated<T> {
    pub total_count: i32,
    pub total_pages: i32,
    pub items: Vec<T>,
}

#[derive(Debug)]
pub struct Artist<'a> {
    id: Cow<'a, str>,
    name: Cow<'a, str>,
}

#[derive(Debug)]
pub struct Song<'a> {
    id: Cow<'a, str>,
    title: Cow<'a, str>,
    artist: Artist<'a>,
    date_added: Cow<'a, str>, // TODO: Date
    series: Option<Cow<'a, str>>,
}

impl<'a, T> From<SearchResultsWrapper<'a>> for Paginated<T>
    where T: From<SearchResult<'a>>
{
    fn from(wrapper: SearchResultsWrapper<'a>) -> Self {
        let items = wrapper.search_result
            .into_iter()
            .map(T::from)
            .collect();

        Paginated {
            total_count: wrapper.total_count,
            total_pages: wrapper.total_page,
            items: items,
        }
    }
}

impl<'a> From<SearchResult<'a>> for Artist<'a> {
    fn from(res: SearchResult<'a>) -> Self {
        Artist {
            id: res.artist_id,
            name: res.artist_name,
        }
    }
}

impl<'a> From<SearchResult<'a>> for Song<'a> {
    fn from(res: SearchResult<'a>) -> Self {
        let series = if res.program_title.is_empty() {
            None
        } else {
            Some(res.program_title)
        };

        Song {
            id: res.req_no,
            title: res.song_name,
            date_added: res.dist_start,
            series: series,
            artist: Artist {
                id: res.artist_id,
                name: res.artist_name,
            },
        }
    }
}

use protocol::{SearchResult, SearchResultsWrapper};
use std::borrow::Cow;
use std::convert::From;

pub struct Paginated<T> {
    page: i32,
    total_count: i32,
    items: Vec<T>,
}

pub struct Artist<'a> {
    id: Cow<'a, str>,
    name: Cow<'a, str>,
}

pub struct Song<'a> {
    id: Cow<'a, str>,
    title: Cow<'a, str>,
    artist: Artist<'a>,
    date_added: Cow<'a, str>, // TODO: Date
    series: Option<Cow<'a, str>>,
}

impl<'a> From<SearchResult<'a>> for Artist<'a> {
    fn from(resp: SearchResult<'a>) -> Self {
        Artist {
            id: resp.artist_id,
            name: resp.artist_name,
        }
    }
}

impl<'a> From<SearchResult<'a>> for Song<'a> {
    fn from(resp: SearchResult<'a>) -> Self {
        let series = if resp.program_title.is_empty() {
            None
        } else {
            Some(resp.program_title)
        };

        Song {
            id: resp.req_no,
            title: resp.song_name,
            date_added: resp.dist_start,
            series: series,
            artist: Artist {
                id: resp.artist_id,
                name: resp.artist_name,
            },
        }
    }
}

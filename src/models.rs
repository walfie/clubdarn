use protocol::{SearchResult, SearchResultsWrapper};
use std::borrow::Cow;
use std::convert::From;

#[derive(Debug)]
pub struct Paginated<'a, T> {
    pub page: i32,
    pub category_id: Cow<'a, str>,
    pub total_count: i32,
    pub total_pages: i32,
    pub items: Vec<T>,
}

#[derive(Debug)]
pub struct Artist<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
}

#[derive(Debug)]
pub struct Song<'a> {
    pub id: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub artist: Artist<'a>,
    pub date_added: Cow<'a, str>,
    pub lyrics: Cow<'a, str>,
}

pub struct Series<'a> {
    pub title: Cow<'a, str>,
    pub first_kana: Cow<'a, str>,
}

impl<'a, T> Paginated<'a, T>
    where T: From<SearchResult<'a>>
{
    pub fn from_results_wrapper(page: i32,
                                category_id: Cow<'a, str>,
                                wrapper: SearchResultsWrapper<'a>)
                                -> Self {
        let items = wrapper.search_result
            .into_iter()
            .map(T::from)
            .collect();

        Paginated {
            page: page,
            category_id: category_id,
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
        Song {
            id: res.req_no,
            title: res.song_name,
            date_added: res.dist_start, // TODO: DateTime
            lyrics: res.first_bars,
            artist: Artist {
                id: res.artist_id,
                name: res.artist_name,
            },
        }
    }
}

impl<'a> From<SearchResult<'a>> for Series<'a> {
    fn from(res: SearchResult<'a>) -> Self {
        Series {
            title: res.program_title,
            first_kana: res.title_first_kana,
        }
    }
}

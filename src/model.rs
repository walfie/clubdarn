use protocol::{SearchResult, SearchResultsWrapper};
use std::borrow::Cow;
use std::convert::From;

#[derive(Debug, Serialize)]
pub struct Paginated<'a, T> {
    pub page: i32,
    pub category_id: Cow<'a, str>,
    pub total_count: i32,
    pub total_pages: i32,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize)]
pub struct Artist<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
}

#[derive(Debug, Serialize)]
pub struct Song<'a> {
    pub id: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub artist: Artist<'a>,
    pub date_added: Cow<'a, str>,
    pub lyrics: Cow<'a, str>,
    pub series: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize)]
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
        let items: Vec<T> = wrapper.search_result
            .into_iter()
            .map(T::from)
            .collect();

        // Sometimes the API says there are multiple pages, but puts all
        // the results on a single page, so we need to manually check.
        let total_pages = if (items.len() as i32) >= wrapper.total_count {
            1
        } else {
            wrapper.total_count
        };

        Paginated {
            page: page,
            category_id: category_id,
            total_count: wrapper.total_count,
            total_pages: total_pages,
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
            date_added: res.dist_start, // TODO: DateTime
            lyrics: res.first_bars,
            series: series,
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

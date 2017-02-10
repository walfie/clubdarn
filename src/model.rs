use protocol::{exist, search};
use std::borrow::Cow;
use std::convert::From;

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

#[derive(Debug, Serialize)]
pub struct Paginated<'a, T: 'a> {
    pub page: i32,
    pub category_id: Option<Cow<'a, str>>,
    pub total_items: i32,
    pub total_pages: i32,
    pub items: Vec<T>,
}

impl<'a, T> Paginated<'a, T> {
    pub fn next_page(&self) -> Option<i32> {
        if self.page < self.total_pages {
            Some(self.page + 1)
        } else {
            None
        }
    }

    pub fn prev_page(&self) -> Option<i32> {
        if self.page > 1 {
            Some(self.page - 1)
        } else {
            None
        }
    }
}

impl<'a> From<search::Item<'a>> for Artist<'a> {
    fn from(res: search::Item<'a>) -> Self {
        Artist {
            id: res.artist_id,
            name: res.artist_name,
        }
    }
}

impl<'a> From<search::Item<'a>> for Song<'a> {
    fn from(res: search::Item<'a>) -> Self {
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

impl<'a> From<exist::Item<'a>> for Song<'a> {
    fn from(res: exist::Item<'a>) -> Self {
        Song {
            id: res.req_no,
            title: res.song_name,
            date_added: res.dist_start, // TODO: DateTime
            lyrics: res.first_bars,
            series: None,
            artist: Artist {
                id: res.artist_id,
                name: res.artist_name,
            },
        }
    }
}

impl<'a> From<search::Item<'a>> for Series<'a> {
    fn from(res: search::Item<'a>) -> Self {
        Series {
            title: res.program_title,
            first_kana: res.title_first_kana,
        }
    }
}

use protocol::{exist, recommend, search};
use std::borrow::Cow;
use std::convert::From;

#[derive(Debug, PartialEq, Serialize)]
pub struct Artist<'a> {
    pub id: i32,
    pub name: Cow<'a, str>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Song<'a> {
    pub id: i32,
    pub title: Cow<'a, str>,
    pub artist: Artist<'a>,
    pub date_added: Option<Cow<'a, str>>,
    pub lyrics: Option<Cow<'a, str>>,
    pub series: Option<Cow<'a, str>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Series<'a> {
    pub title: Cow<'a, str>,
    pub first_kana: Cow<'a, str>,
}

#[derive(Debug, PartialEq, Serialize)]
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

fn parse_str_id(id: Cow<str>) -> i32 {
    id.parse().unwrap_or(-1)
}

impl<'a> From<search::Item<'a>> for Artist<'a> {
    fn from(res: search::Item<'a>) -> Self {
        Artist {
            id: parse_str_id(res.artist_id),
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
            id: parse_str_id(res.req_no),
            title: res.song_name,
            date_added: Some(res.dist_start), // TODO: DateTime
            lyrics: Some(res.first_bars),
            series: series,
            artist: Artist {
                id: parse_str_id(res.artist_id),
                name: res.artist_name,
            },
        }
    }
}

impl<'a> From<exist::Item<'a>> for Song<'a> {
    fn from(res: exist::Item<'a>) -> Self {
        Song {
            id: parse_str_id(res.req_no),
            title: res.song_name,
            date_added: Some(res.dist_start), // TODO: DateTime
            lyrics: Some(res.first_bars),
            series: None,
            artist: Artist {
                id: parse_str_id(res.artist_id),
                name: res.artist_name,
            },
        }
    }
}

impl<'a> From<recommend::Item<'a>> for Song<'a> {
    fn from(res: recommend::Item<'a>) -> Self {
        Song {
            // TODO: `replacen` stabilizes in Rust 1.16.0
            id: parse_str_id(res.request_no.replace("-", "").into()),
            title: res.denmoku_contents,
            date_added: None,
            lyrics: None,
            series: None,
            artist: Artist {
                id: parse_str_id(res.dam_artist_code),
                name: res.artist,
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

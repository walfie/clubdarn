use protocol::{exist, recommend, search};
use std::borrow::Cow;
use std::convert::From;

#[derive(Debug, PartialEq, Serialize)]
pub struct SongId(pub i32);
#[derive(Debug, PartialEq, Serialize)]
pub struct ArtistId(pub i32);

#[derive(Debug, PartialEq, Serialize)]
pub struct Artist<'a> {
    pub id: ArtistId,
    pub name: Cow<'a, str>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Song<'a> {
    pub id: SongId,
    pub title: Cow<'a, str>,
    pub artist: Artist<'a>,
    #[serde(rename = "dateAdded", skip_serializing_if = "Option::is_none")]
    pub date_added: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lyrics: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Cow<'a, str>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Series<'a> {
    pub title: Cow<'a, str>,
    #[serde(rename = "firstKana")]
    pub first_kana: Cow<'a, str>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Paginated<'a, T: 'a> {
    pub page: i32,
    #[serde(rename="categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<Cow<'a, str>>,
    #[serde(rename="total_items")]
    pub total_items: i32,
    #[serde(rename="total_pages")]
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

impl<'a> From<Cow<'a, str>> for SongId {
    fn from(s: Cow<'a, str>) -> Self {
        // TODO: `replacen` stabilizes in Rust 1.16.0
        SongId(s.replace("-", "").parse().unwrap_or(-1))
    }
}

impl<'a> From<Cow<'a, str>> for ArtistId {
    fn from(s: Cow<'a, str>) -> Self {
        ArtistId(s.parse().unwrap_or(-1))
    }
}

impl<'a> From<search::Item<'a>> for Artist<'a> {
    fn from(res: search::Item<'a>) -> Self {
        Artist {
            id: res.artist_id.into(),
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
            id: res.req_no.into(),
            title: res.song_name,
            date_added: Some(res.dist_start), // TODO: DateTime
            lyrics: Some(res.first_bars),
            series: series,
            artist: Artist {
                id: res.artist_id.into(),
                name: res.artist_name,
            },
        }
    }
}

impl<'a> From<exist::Item<'a>> for Song<'a> {
    fn from(res: exist::Item<'a>) -> Self {
        Song {
            id: res.req_no.into(),
            title: res.song_name,
            date_added: Some(res.dist_start), // TODO: DateTime
            lyrics: Some(res.first_bars),
            series: None,
            artist: Artist {
                id: res.artist_id.into(),
                name: res.artist_name,
            },
        }
    }
}

impl<'a> From<recommend::Item<'a>> for Song<'a> {
    fn from(res: recommend::Item<'a>) -> Self {
        Song {
            id: res.request_no.into(),
            title: res.denmoku_contents,
            date_added: None,
            lyrics: None,
            series: None,
            artist: Artist {
                id: res.dam_artist_code.into(),
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

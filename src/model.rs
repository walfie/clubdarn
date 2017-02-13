use protocol::{exist, recommend, search};
use std::borrow::Cow;
use std::convert::From;
use std::ops::Not;

#[derive(Debug, PartialEq, Serialize)]
pub struct SongId(pub i32);
#[derive(Debug, PartialEq, Serialize)]
pub struct ArtistId(pub i32);

#[derive(Debug, PartialEq, Serialize)]
pub struct Artist {
    pub id: ArtistId,
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Song {
    pub id: SongId,
    pub title: String,
    pub artist: Artist,
    #[serde(rename = "dateAdded", skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lyrics: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(rename = "hasVideo", skip_serializing_if = "Not::not")]
    pub has_video: bool,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Series {
    pub title: String,
    #[serde(rename = "firstKana")]
    pub first_kana: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Paginated<T> {
    pub page: i32,
    #[serde(rename="artistCategoryId")]
    pub artist_category_id: String,
    #[serde(rename="seriesCategoryId")]
    pub series_category_id: Option<String>,
    #[serde(rename="total_items")]
    pub total_items: i32,
    #[serde(rename="total_pages")]
    pub total_pages: i32,
    pub items: Vec<T>,
}

impl<T> Paginated<T> {
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

impl<'a, T> From<T> for SongId
    where T: Into<Cow<'a, str>>
{
    fn from(s: T) -> Self {
        // TODO: `replacen` stabilizes in Rust 1.16.0
        SongId(s.into().replace("-", "").parse().unwrap_or(-1))
    }
}

impl<'a, T> From<T> for ArtistId
    where T: Into<Cow<'a, str>>
{
    fn from(s: T) -> Self {
        ArtistId(s.into().parse().unwrap_or(-1))
    }
}

impl From<search::Item> for Artist {
    fn from(res: search::Item) -> Self {
        Artist {
            id: res.artist_id.into(),
            name: res.artist_name,
        }
    }
}

impl From<search::Item> for Song {
    fn from(res: search::Item) -> Self {
        let series = if res.program_title.is_empty() {
            None
        } else {
            Some(res.program_title)
        };

        let has_video = res.func_anime_picture == "1" || res.func_person_picture == "1";

        Song {
            id: res.req_no.into(),
            title: res.song_name,
            date_added: Some(res.dist_start), // TODO: DateTime
            lyrics: Some(res.first_bars),
            series: series,
            has_video: has_video,
            artist: Artist {
                id: res.artist_id.into(),
                name: res.artist_name,
            },
        }
    }
}

impl From<exist::Item> for Song {
    fn from(res: exist::Item) -> Self {
        let has_video = res.func_anime_picture == "1" || res.func_person_picture == "1";

        Song {
            id: res.req_no.into(),
            title: res.song_name,
            date_added: Some(res.dist_start), // TODO: DateTime
            lyrics: Some(res.first_bars),
            series: None,
            has_video: has_video,
            artist: Artist {
                id: res.artist_id.into(),
                name: res.artist_name,
            },
        }
    }
}

impl From<recommend::Item> for Song {
    fn from(res: recommend::Item) -> Self {
        Song {
            id: res.request_no.into(),
            title: res.denmoku_contents,
            date_added: None,
            lyrics: None,
            series: None,
            has_video: false,
            artist: Artist {
                id: res.dam_artist_code.into(),
                name: res.artist,
            },
        }
    }
}

impl From<search::Item> for Series {
    fn from(res: search::Item) -> Self {
        Series {
            title: res.program_title,
            first_kana: res.title_first_kana,
        }
    }
}

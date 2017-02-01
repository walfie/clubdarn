pub mod categories;

use std::borrow::Cow;

#[derive(Default, Serialize)]
pub struct DkDamSearchServletRequest<'a> {
    #[serde(rename = "appVer")]
    pub app_ver: Cow<'a, str>,
    #[serde(rename = "deviceId")]
    pub device_id: Cow<'a, str>,
    #[serde(rename = "deviceNm")]
    pub device_nm: Cow<'a, str>,
    #[serde(rename = "osVer")]
    pub os_ver: Cow<'a, str>,
    #[serde(rename = "serialNo", skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<Cow<'a, str>>,

    #[serde(rename = "page")]
    pub page: Cow<'a, str>,
    #[serde(rename = "categoryCd")]
    pub category_cd: Cow<'a, str>,

    #[serde(rename = "artistId", skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<Cow<'a, str>>,

    #[serde(rename = "artistName", skip_serializing_if = "Option::is_none")]
    pub artist_name: Option<Cow<'a, str>>,
    #[serde(rename = "artistMatchType", skip_serializing_if = "Option::is_none")]
    pub artist_match_type: Option<Cow<'a, str>>,

    #[serde(rename = "songName", skip_serializing_if = "Option::is_none")]
    pub song_name: Option<Cow<'a, str>>,
    #[serde(rename = "songMatchType", skip_serializing_if = "Option::is_none")]
    pub song_match_type: Option<Cow<'a, str>>,

    #[serde(rename = "programTitle", skip_serializing_if = "Option::is_none")]
    pub program_title: Option<Cow<'a, str>>
}

pub struct MatchType(pub &'static str);
pub const STARTS_WITH: MatchType = MatchType("0");
pub const CONTAINS: MatchType = MatchType("1");

impl<'a> DkDamSearchServletRequest<'a> {
    pub fn new() -> Self {
        DkDamSearchServletRequest { page: "1".into(), .. Default::default() }
    }

    pub fn serial_no(&mut self, serial_no: &'a str) -> &mut Self {
       self.serial_no = Some(serial_no.into());
       self
    }

    pub fn page(&mut self, page: i8) -> &mut Self {
        self.page = page.to_string().into();
        self
    }

    pub fn by_artist_id(&mut self, id: &'a str) -> &mut Self {
        self.artist_id = Some(id.into());
        self.category_cd = categories::ARTIST_NAME.0.into();
        self
    }

    pub fn by_artist_name(&mut self, name: &'a str, match_type: MatchType) -> &mut Self {
        self.artist_name = Some(name.into());
        self.artist_match_type = Some(match_type.0.into());
        self.category_cd = categories::ARTIST_NAME.0.into();
        self
    }

    pub fn by_song_name(&mut self, name: &'a str, match_type: MatchType) -> &mut Self {
        self.song_name = Some(name.into());
        self.song_match_type = Some(match_type.0.into());
        self.category_cd = categories::SONG_NAME.0.into();
        self
    }

    // Anime only
    pub fn by_program_title(&mut self, title: &'a str) -> &mut Self {
        self.program_title = Some(title.into());
        self.category_cd = categories::ANIMATION_SPECIAL_EFFECTS_ANIME.0.into();
        self
    }
}


use protocol::DkDamSearchServletRequest;
use protocol::categories;

extern crate serde_json;

pub struct Client<'a> {
    pub app_ver: &'a str,
    pub device_id: &'a str,
    pub device_nm: &'a str,
    pub os_ver: &'a str,
    pub serial_no: Option<&'a str>,
}

pub struct MatchType(pub &'static str);
pub const STARTS_WITH: MatchType = MatchType("0");
pub const CONTAINS: MatchType = MatchType("1");

pub struct Request<'a>(DkDamSearchServletRequest<'a>);
impl<'a> Request<'a> {
    pub fn page(&mut self, page_num: i8) -> &Self {
        self.0.page = page_num;
        self
    }
}

impl<'a> Client<'a> {
    fn default_request(&self) -> DkDamSearchServletRequest {
        DkDamSearchServletRequest {
            app_ver: self.app_ver,
            device_id: self.device_id,
            device_nm: self.device_nm,
            os_ver: self.os_ver,
            serial_no: self.serial_no,
            page: 1,
            ..Default::default()
        }
    }

    pub fn songs_by_artist_id(&self, id: &'a str) -> String {
        let mut req = self.default_request();
        req.artist_id = Some(id);
        req.category_cd = categories::ARTIST_NAME.0;
        make_request(req)
    }

    pub fn songs_by_title(&self, title: &'a str, match_type: MatchType) -> String {
        let mut req = self.default_request();
        req.song_name = Some(title);
        req.category_cd = categories::SONG_NAME.0;
        req.artist_match_type = Some(match_type.0);
        make_request(req)
    }

    pub fn artists_by_name(&self, name: &'a str, match_type: MatchType) -> String {
        let mut req = self.default_request();
        req.artist_name = Some(name);
        req.category_cd = categories::ARTIST_NAME.0;
        req.artist_match_type = Some(match_type.0);
        make_request(req)
    }

    pub fn songs_by_series(&self, title: &'a str) -> String {
        let mut req = self.default_request();
        req.program_title = Some(title);
        req.category_cd = categories::ANIMATION_SPECIAL_EFFECTS_ANIME.0;
        make_request(req)
    }
}

fn make_request(req: DkDamSearchServletRequest) -> String {
    serde_json::to_string_pretty(&req).unwrap() // TODO
}

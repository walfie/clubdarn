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
        make_request(DkDamSearchServletRequest {
            artist_id: Some(id),
            category_cd: categories::ARTIST_NAME.0,
            ..self.default_request()
        })
    }
}

fn make_request(req: DkDamSearchServletRequest) -> String {
    serde_json::to_string_pretty(&req).unwrap() // TODO
}

#[derive(Default, Serialize)]
pub struct DkDamSearchServletRequest {
    #[serde(rename = "appVer")]
    pub app_ver: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "deviceNm")]
    pub device_nm: String,
    #[serde(rename = "osVer")]
    pub os_ver: String,

    #[serde(rename = "page")]
    pub page: String,
    #[serde(rename = "categoryCd")]
    pub category_cd: String,

    #[serde(rename = "serialNo", skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,

    #[serde(rename = "artistId", skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<String>,

    #[serde(rename = "artistName", skip_serializing_if = "Option::is_none")]
    pub artist_name: Option<String>,
    #[serde(rename = "artistMatchType", skip_serializing_if = "Option::is_none")]
    pub artist_match_type: Option<String>,

    #[serde(rename = "songName", skip_serializing_if = "Option::is_none")]
    pub song_name: Option<String>,
    #[serde(rename = "songMatchType", skip_serializing_if = "Option::is_none")]
    pub song_match_type: Option<String>,

    #[serde(rename = "programTitle", skip_serializing_if = "Option::is_none")]
    pub program_title: Option<String>
}


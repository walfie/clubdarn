pub mod categories;

use serde::Serializer;

fn serialize_i8_as_str<S>(n: &i8, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    serializer.serialize_str((*n).to_string().as_str())
}

#[derive(Default, Serialize)]
pub struct DkDamSearchServletRequest<'a> {
    #[serde(rename = "appVer")]
    pub app_ver: &'a str,
    #[serde(rename = "deviceId")]
    pub device_id: &'a str,
    #[serde(rename = "deviceNm")]
    pub device_nm: &'a str,
    #[serde(rename = "osVer")]
    pub os_ver: &'a str,
    #[serde(rename = "serialNo", skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<&'a str>,

    #[serde(rename = "page", serialize_with = "serialize_i8_as_str")]
    pub page: i8,
    #[serde(rename = "categoryCd")]
    pub category_cd: &'a str,

    #[serde(rename = "artistId", skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<&'a str>,

    #[serde(rename = "artistName", skip_serializing_if = "Option::is_none")]
    pub artist_name: Option<&'a str>,
    #[serde(rename = "artistMatchType", skip_serializing_if = "Option::is_none")]
    pub artist_match_type: Option<&'a str>,

    #[serde(rename = "songName", skip_serializing_if = "Option::is_none")]
    pub song_name: Option<&'a str>,
    #[serde(rename = "songMatchType", skip_serializing_if = "Option::is_none")]
    pub song_match_type: Option<&'a str>,

    #[serde(rename = "programTitle", skip_serializing_if = "Option::is_none")]
    pub program_title: Option<&'a str>,
}

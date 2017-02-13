use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Serialize)]
pub struct CategoryId<'a>(pub Cow<'a, str>);

pub trait CategoryType {}

pub struct SongCategory;
impl CategoryType for SongCategory {}

pub struct ArtistCategory;
impl CategoryType for ArtistCategory {}

pub struct SeriesCategory;
impl CategoryType for SeriesCategory {}

#[derive(Serialize)]
pub struct Category<T: CategoryType> {
    pub id: CategoryId<'static>,
    pub description: Description,
    #[serde(skip_serializing)]
    category_type: PhantomData<T>,
}

#[derive(Serialize)]
pub struct Description {
    pub en: &'static str,
    pub ja: &'static str,
}

macro_rules! count_items {
    ($name:ident) => { 1 };
    ($first:ident, $($rest:ident),*) => {
        1 + count_items!($($rest),*)
    }
}

macro_rules! category {
    ($cat_type:ident, $cat_id:expr, $cat_name:ident, $desc_ja:expr, $desc_en:expr) => {
        pub const $cat_name: Category<$cat_type> = Category {
            id: CategoryId(::std::borrow::Cow::Borrowed($cat_id)),
            description: Description {
                en: $desc_en,
                ja: $desc_ja,
            },
            category_type: ::std::marker::PhantomData,
        };
    }
}

macro_rules! categories {
    (
        $($mod_name:ident<$cat_type:ident> [
            $( ( $cat_id:expr, $cat_name:ident, $desc_ja:expr, $desc_en:expr ), )+
        ],)+
    ) => { $(
        pub mod $mod_name {
            use super::*;

            $( category!($cat_type, $cat_id, $cat_name, $desc_ja, $desc_en); )+

            pub const ALL_CATEGORIES: [Category<$cat_type>; count_items!( $($cat_name),+ )] =
                [$( $cat_name ),+];
        }
    )+ };
}

category!(ArtistCategory,
          "010000",
          ARTIST_NAME,
          "歌手名",
          "Artist Name");
category!(SongCategory, "020000", SONG_NAME, "曲名", "Song Name");
category!(ArtistCategory,
          "040000",
          CAST_PICTURE,
          "本人映像",
          "Cast Picture");

categories! [
    new_songs<SongCategory> [
        ("030100", ALL, "全曲", ""),
        ("030201", LIVE_KARAOKE, "LIVEカラオケ", ""),
        ("030202", CAST_PICTURE, "本人出演映像", ""),
        ("030203", CLIP_JUST_NOW, "今だけクリップ", ""),
        ("030301", ANIME_GAME, "アニメ・ゲーム", ""),
        ("030302", TOKUSATSU, "特撮", ""),
        ("030401", CM, "CM", ""),
        ("030402", DRAMA_MOVIE, "ドラマ・映画", ""),
        ("030403", VARIETY, "バラエティ", ""),
        ("030404", MUSIC_PROGRAM, "音楽番組", ""),
        ("030405", INFORMATION_PROGRAM, "情報番組", ""),
        ("030406", SPORTS, "スポーツ", ""),
        ("030500", SOON_DELIVERY, "もうすぐ配信", ""),
    ],
    anime<SeriesCategory> [
        ("050100", ANIME, "アニメ", ""),
        ("050200", TOKUSATSU, "特撮", ""),
        ("050300", IMAGE, "映像", ""),
    ],
    vocaloid<SongCategory> [
        ("060100", MIKU, "初音ミク", ""),
        ("060200", RIN_LEN, "鏡音リン・鏡音レン", ""),
        ("060300", LUKA, "巡音ルカ", ""),
        ("060400", KAITO_MEIKO, "KAITO・MEIKO", ""),
        ("060500", GUMI, "GUMI", ""),
        ("060600", GAKUPO, "神威がくぽ", ""),
        ("060700", LILY, "Lily", ""),
        ("060800", OTHER, "その他", ""),
        ("060900", IMAGE, "映像", ""),
    ],
    ranking<SongCategory> [
        ("070100", POP, "POPS", ""),
        ("070200", BALLAD, "演歌", ""),
        ("070300", WESTERN, "洋楽", ""),
        ("070400", DUET, "デュエット", ""),
        ("070500", ANIME_TOKUSATSU, "アニメ・特撮", ""),
        ("071100", RECOMMENDED1, "", ""),
        ("071200", RECOMMENDED2, "", ""),
        ("071300", RECOMMENDED3, "", ""),
        ("071400", RECOMMENDED4, "", ""),
        ("071500", RECOMMENDED5, "", ""),
    ],
];

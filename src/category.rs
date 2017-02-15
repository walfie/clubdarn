use std::marker::PhantomData;

#[derive(Serialize)]
pub struct CategoryId<'a>(pub &'a str);

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
            id: CategoryId($cat_id),
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

            pub const CATEGORIES: [Category<$cat_type>; count_items!( $($cat_name),+ )] =
                [$( $cat_name ),+];
        }
    )+ };
}

category!(SongCategory, "020000", SONG_NAME, "曲名", "Song Name");
category!(ArtistCategory,
          "010000",
          ARTIST_NAME,
          "歌手名",
          "Artist Name");
category!(ArtistCategory,
          "040000",
          LIVE_PERFORMANCE,
          "本人映像",
          "Live Performance");

categories! [
    new_songs<SongCategory> [
        ("030100", ALL, "全曲", "All"),
        ("030201", LIVE_KARAOKE, "LIVEカラオケ", "Live Karaoke"),
        ("030202", MUSIC_VIDEO, "本人出演映像", "Music Video"),
        ("030203", LIMITED_TIME_VIDEO, "今だけクリップ", "Limited Time Video"),
        ("030301", ANIME_GAME, "アニメ・ゲーム", "Anime/Game"),
        ("030302", TOKUSATSU, "特撮", "Tokusatsu"),
        ("030401", CM, "CM", "Commercial"),
        ("030402", DRAMA_MOVIE, "ドラマ・映画", "Drama/Movie"),
        ("030403", VARIETY, "バラエティ", "Variety"),
        ("030404", MUSIC_PROGRAM, "音楽番組", "Music Program"),
        ("030405", INFORMATION_PROGRAM, "情報番組", "Information Program"),
        ("030406", SPORTS, "スポーツ", "Sports"),
        ("030500", COMING_SOON, "もうすぐ配信", "Coming Soon"),
    ],
    series<SeriesCategory> [
        ("050100", ANIME, "アニメ", "Anime"),
        ("050200", TOKUSATSU, "特撮", "Tokusatsu"),
        ("050300", MUSIC_VIDEO, "映像", "Music Video"),
    ],
    vocaloid<SongCategory> [
        ("060100", MIKU, "初音ミク", "Miku"),
        ("060200", RIN_LEN, "鏡音リン・鏡音レン", "Rin/Len"),
        ("060300", LUKA, "巡音ルカ", "Luka"),
        ("060400", KAITO_MEIKO, "KAITO・MEIKO", "Kaito/Meiko"),
        ("060500", GUMI, "GUMI", "GUMI"),
        ("060600", GAKUPO, "神威がくぽ", "Gakupo"),
        ("060700", LILY, "Lily", "Lily"),
        ("060800", OTHER, "その他", "Other"),
        ("060900", IMAGE, "映像", "Music Video"),
    ],
    ranking<SongCategory> [
        ("070100", POP, "POPS", "Pop"),
        ("070200", BALLAD, "演歌", "Ballad"),
        ("070300", WESTERN, "洋楽", "Western"),
        ("070400", DUET, "デュエット", "Duet"),
        ("070500", ANIME_TOKUSATSU, "アニメ・特撮", "Anime/Tokusatsu"),
        ("071100", RECOMMENDED1, "Recommended (1)", "Recommended (1)"),
        ("071200", RECOMMENDED2, "Recommended (2)", "Recommended (2)"),
        ("071300", RECOMMENDED3, "Recommended (3)", "Recommended (3)"),
        ("071400", RECOMMENDED4, "Recommended (4)", "Recommended (4)"),
        ("071500", RECOMMENDED5, "Recommended (5)", "Recommended (5)"),
    ],
];

pub fn series_category<'a>(input: &'a str) -> Option<Category<SeriesCategory>> {
    if input == new_songs::ANIME_GAME.id.0 || input == series::ANIME.id.0 ||
       input == ranking::ANIME_TOKUSATSU.id.0 {
        Some(series::ANIME)
    } else if input == new_songs::TOKUSATSU.id.0 || input == series::TOKUSATSU.id.0 {
        Some(series::TOKUSATSU)
    } else if input == series::MUSIC_VIDEO.id.0 {
        Some(series::MUSIC_VIDEO)
    } else {
        None
    }
}

pub fn artist_category<'a>(input: &'a str) -> Category<ArtistCategory> {
    if input == LIVE_PERFORMANCE.id.0 {
        LIVE_PERFORMANCE
    } else {
        ARTIST_NAME
    }
}

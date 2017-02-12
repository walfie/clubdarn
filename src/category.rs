use std::borrow::Cow;

pub struct CategoryId<'a>(pub Cow<'a, str>);

impl<'a> From<CategoryId<'a>> for Cow<'a, str> {
    fn from(c: CategoryId<'a>) -> Self {
        c.0
    }
}

pub enum Other {
    ArtistName,
    SongName,
    CastPicture,
}

impl<'a> From<Other> for CategoryId<'a> {
    fn from(o: Other) -> Self {
        use self::Other::*;

        let id = match o {
            SongName => "020000", // 曲名
            ArtistName => "010000", // 歌手名
            CastPicture => "040000", // 本人映像
        };
        CategoryId(id.into())
    }
}

pub enum NewSong {
    All,
    LiveKaraoke,
    CastPicture,
    ClipJustNow,
    AnimeGame,
    Tokusatsu,
    Cm,
    DramaMovie,
    Variety,
    MusicProgram,
    InformationProgram,
    Sports,
    SoonDelivery,
}

impl<'a> From<NewSong> for CategoryId<'a> {
    fn from(n: NewSong) -> Self {
        use self::NewSong::*;

        let id = match n {
            // ジャンル>新曲
            All => "030100", // 全曲

            // ジャンル>新曲>映像
            LiveKaraoke => "030201", // LIVEカラオケ
            CastPicture => "030202", // 本人出演映像
            ClipJustNow => "030203", // 今だけクリップ

            // ジャンル>新曲>ｱﾆﾒ・特撮
            AnimeGame => "030301", // アニメ・ゲーム
            Tokusatsu => "030302", // 特撮
            Cm => "030401", // CM

            // ジャンル>新曲>TV・映画・スポーツ
            DramaMovie => "030402", // ドラマ・映画
            Variety => "030403", // バラエティ
            MusicProgram => "030404", // 音楽番組
            InformationProgram => "030405", // 情報番組
            Sports => "030406", // スポーツ
            SoonDelivery => "030500", // もうすぐ配信
        };
        CategoryId(id.into())
    }
}

pub enum Series {
    Anime,
    Tokusatsu,
    Image,
}

impl<'a> From<Series> for CategoryId<'a> {
    fn from(s: Series) -> Self {
        use self::Series::*;

        let id = match s {
            // ｱﾆﾒ･特撮
            Anime => "050100", // アニメ
            Tokusatsu => "050200", // 特撮
            Image => "050300", // 映像
        };
        CategoryId(id.into())
    }
}

pub enum Vocaloid {
    Miku,
    RinLen,
    Luka,
    KaitoMeiko,
    Gumi,
    Gakupo,
    Lily,
    Other,
    Image,
}

impl<'a> From<Vocaloid> for CategoryId<'a> {
    fn from(v: Vocaloid) -> Self {
        use self::Vocaloid::*;

        let id = match v {
            // ジャンル>ﾎﾞｰｶﾛｲﾄﾞ
            Miku => "060100", // 初音ミク
            RinLen => "060200", // 鏡音リン・鏡音レン
            Luka => "060300", // 巡音ルカ
            KaitoMeiko => "060400", // KAITO・MEIKO
            Gumi => "060500", // GUMI
            Gakupo => "060600", // 神威がくぽ
            Lily => "060700", // Lily
            Other => "060800", // その他
            Image => "060900", // 映像

        };
        CategoryId(id.into())
    }
}

pub enum Ranking {
    Pop,
    Ballad,
    Western,
    Duet,
    AnimeTokusatsu,
    Recommended1,
    Recommended2,
    Recommended3,
    Recommended4,
    Recommended5,
}

impl<'a> From<Ranking> for CategoryId<'a> {
    fn from(r: Ranking) -> Self {
        use self::Ranking::*;

        let id = match r {
            // ランキング
            Pop => "070100", // POPS
            Ballad => "070200", // 演歌
            Western => "070300", // 洋楽
            Duet => "070400", // デュエット
            AnimeTokusatsu => "070500", // アニメ・特撮
            Recommended1 => "071100",
            Recommended2 => "071200",
            Recommended3 => "071300",
            Recommended4 => "071400",
            Recommended5 => "071500",
        };
        CategoryId(id.into())
    }
}

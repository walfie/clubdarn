pub struct CategoryId(pub &'static str);

pub const ARTIST_NAME: CategoryId = CategoryId("010000");
pub const SONG_NAME: CategoryId = CategoryId("020000");

// ジャンル>新曲
pub const NEW_SONG_ALL_SONG: CategoryId = CategoryId("030100"); // 全曲

// ジャンル>新曲>映像
pub const NEW_SONG_LIVE_KARAOKE: CategoryId = CategoryId("030201"); // LIVEカラオケ
pub const NEW_SONG_CAST_PICTURE: CategoryId = CategoryId("030202"); // 本人出演映像
pub const NEW_SONG_CLIP_JUST_NOW: CategoryId = CategoryId("030203"); // 今だけクリップ

// programTitle included, can't search by it
// ジャンル>新曲>ｱﾆﾒ・特撮
pub const NEW_SONG_ANIME_GAME: CategoryId = CategoryId("030301"); // アニメ・ゲーム
pub const NEW_SONG_SPECIAL_EFFECTS: CategoryId = CategoryId("030302"); // 特撮

// programTitle included, can't search by it
// ジャンル>新曲>TV・映画・スポーツ
pub const NEW_SONG_CM: CategoryId = CategoryId("030401"); // CM
pub const NEW_SONG_DRAMA_MOVIE: CategoryId = CategoryId("030402"); // ドラマ・映画
pub const NEW_SONG_VARIETY: CategoryId = CategoryId("030403"); // バラエティ
pub const NEW_SONG_MUSIC_PROGRAM: CategoryId = CategoryId("030404"); // 音楽番組
pub const NEW_SONG_INFORMATION_PROGRAM: CategoryId = CategoryId("030405"); // 情報番組
pub const NEW_SONG_SPORTS: CategoryId = CategoryId("030406"); // スポーツ
pub const NEW_SONG_SOON_DELIVERY: CategoryId = CategoryId("030500"); // もうすぐ配信

// Get artist, use artistId and same category to search for songs
// ジャンル
pub const CAST_PICTURE: CategoryId = CategoryId("040000"); // 本人映像

// programTitle included (intended on only using programTitle)
// ｱﾆﾒ･特撮
pub const ANIMATION_SPECIAL_EFFECTS_ANIME: CategoryId = CategoryId("050100"); // アニメ
pub const ANIMATION_SPECIAL_EFFECTS_SPECIAL: CategoryId = CategoryId("050200"); // 特撮
pub const ANIMATION_SPECIAL_EFFECTS_IMAGE: CategoryId = CategoryId("050300"); // 映像

// Returns all songs
// ジャンル>ﾎﾞｰｶﾛｲﾄﾞ
pub const VOCALOID_HATSUNE_MIKU: CategoryId = CategoryId("060100"); // 初音ミク
pub const VOCALOID_KAGAMINE_RIN_REN: CategoryId = CategoryId("060200"); // 鏡音リン・鏡音レン
pub const VOCALOID_MEGURINE_RUKA: CategoryId = CategoryId("060300"); // 巡音ルカ
pub const VOCALOID_KAITO_MEIKO: CategoryId = CategoryId("060400"); // KAITO・MEIKO
pub const VOCALOID_GUMI: CategoryId = CategoryId("060500"); // GUMI
pub const VOCALOID_KAMUI_GAKUPO: CategoryId = CategoryId("060600"); // 神威がくぽ
pub const VOCALOID_LILY: CategoryId = CategoryId("060700"); // Lily
pub const VOCALOID_OTHER: CategoryId = CategoryId("060800"); // その他
pub const VOCALOID_IMAGE: CategoryId = CategoryId("060900"); // 映像

// ランキング
pub const DAM_BEST_POPS: CategoryId = CategoryId("070100"); // POPS
pub const DAM_BEST_BALLAD: CategoryId = CategoryId("070200"); // 演歌
pub const DAM_BEST_WESTERN_MUSIC: CategoryId = CategoryId("070300"); // 洋楽
pub const DAM_BEST_DUET: CategoryId = CategoryId("070400"); // デュエット
pub const DAM_BEST_ANIME_SPECIAL: CategoryId = CategoryId("070500"); // アニメ・特撮

pub const DAM_BEST_RECOMMENDED_1: CategoryId = CategoryId("071100");
pub const DAM_BEST_RECOMMENDED_2: CategoryId = CategoryId("071200");
pub const DAM_BEST_RECOMMENDED_3: CategoryId = CategoryId("071300");
pub const DAM_BEST_RECOMMENDED_4: CategoryId = CategoryId("071400");
pub const DAM_BEST_RECOMMENDED_5: CategoryId = CategoryId("071500");

// コンテンツ
pub const CNTNTS_RANKING_BATTLE: CategoryId = CategoryId("080100"); // ランキングバトルりれき
pub const CNTNTS_PRECISION_GRADING: CategoryId = CategoryId("080200"); // 精密採点りれき
pub const CNTNTS_PRECISION_GRADING_II: CategoryId = CategoryId("080300"); // 精密採点Ⅱりれき
pub const CNTNTS_PRECISION_GRADING_DX: CategoryId = CategoryId("080400"); // 精密採点DXりれき
pub const CNTNTS_FULL_CHORUS: CategoryId = CategoryId("080500"); // 完唱!歌いきりまショー!!りれき
pub const CNTNTS_OTHER_HISTORY: CategoryId = CategoryId("080600"); // その他採点りれき
pub const CNTNTS_VOICE_TRAINING_LOG: CategoryId = CategoryId("080700"); // ボイストレーニングりれき
pub const CNTNTS_PRECISION_GRADING_DX_G: CategoryId = CategoryId("080800"); // 精密採点DX-Gりれき
pub const CNTNTS_PRECISION_GRADING_DX_DUET: CategoryId = CategoryId("080900"); // 精密採点DXﾃﾞｭｴｯﾄりれき

// TODO: Rename these
pub const CNTNTS_RnkBtlXG5: CategoryId = CategoryId("810022"); // ランキングバトル
pub const CNTNTS_RnkBtlXG1: CategoryId = CategoryId("810017"); // ランキングバトル
pub const CNTNTS_TnmBtl: CategoryId = CategoryId("810012"); // 勝ち抜きバトル
pub const CNTNTS_STL_F: CategoryId = CategoryId("810013"); // 完唱!歌いきりまショー!!(初代)
pub const CNTNTS_STL_N: CategoryId = CategoryId("810023"); // 完唱!歌いきりまショー!!(通常)
pub const CNTNTS_STL_H: CategoryId = CategoryId("810024"); // 完唱!歌いきりまショー!!(激辛)
pub const CNTNTS_DetailDx: CategoryId = CategoryId("810021"); // 精密採点DX
pub const CNTNTS_Detail2: CategoryId = CategoryId("810014"); // 精密採点Ⅱ
pub const CNTNTS_Detail: CategoryId = CategoryId("810006"); // 精密採点
pub const CNTNTS_Simple: CategoryId = CategoryId("810020"); // シンプル採点
pub const CNTNTS_RedWht: CategoryId = CategoryId("810025"); // カラオケ紅白歌合戦
pub const CNTNTS_Mikawa: CategoryId = CategoryId("810028"); // 美川憲一のアンタ、歌えんの!?
pub const CNTNTS_DetailDxG: CategoryId = CategoryId("810039"); // 精密採点DX-G
pub const CNTNTS_OnePiece: CategoryId = CategoryId("810042"); // ONE PIECE 採点
pub const CNTNTS_VarietyOmoshiro: CategoryId = CategoryId("810041"); // バラエティカラオケ おもしろコース
pub const CNTNTS_VarietyUtauma: CategoryId = CategoryId("810044"); // バラエティカラオケ 歌うまコース
pub const CNTNTS_End: CategoryId = CategoryId("810099"); // 採点ゲームおわり

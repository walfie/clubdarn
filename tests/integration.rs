extern crate clubdarn;

use clubdarn::*;

#[test]
fn songs_by_title() {
    let client = Client::default();

    // TODO: Make it so `.songs()` doesn't need to live as long as `starting_with(...)`
    let song_request = client.songs();
    let mut request = song_request.starting_with("wake up my music");

    // "Wake up my music" should exist on LiveDAM...
    let response = request.send();

    let expected_song = Song {
        id: SongId(366869),
        title: "Wake up my music".into(),
        date_added: Some("2013/11/02".into()),
        end_date: None,
        lyrics: Some("毎日違うわたしに 気づいてるかな".into()),
        series: None,
        first_kana: None,
        has_video: false,
        artist: Artist {
            id: ArtistId(96028),
            name: "りさ、えいみ".into(),
            first_kana: None,
        },
    };

    let expected_response = Paginated {
        page: 1,
        artist_category_id: ::category::ARTIST_NAME.id.0.into(),
        series_category_id: None,
        total_items: 1,
        total_pages: 1,
        items: vec![expected_song],
    };

    assert_eq!(response, expected_response);

    // ... but not on Premier DAM
    let response_empty = request.set_serial_no(Some("AB316238")).send();
    assert_eq!(response_empty.items.is_empty(), true);
}

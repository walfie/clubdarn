extern crate clubdarn;

use clubdarn::*;

#[test]
fn songs_by_title() {
    let client = Client::default();

    let mut request = client.songs_by_title("wake up my music", MatchType::StartsWith);

    // "Wake up my music" should exist on LiveDAM...
    let response = request.send();

    let expected_song = Song {
        id: "366869".into(),
        title: "Wake up my music".into(),
        date_added: "20131102".into(),
        lyrics: "毎日違うわたしに 気づいてるかな".into(),
        series: None,
        artist: Artist {
            id: "96028".into(),
            name: "りさ、えいみ".into(),
        },
    };

    let expected_response = Paginated {
        page: 1,
        category_id: Some("020000".into()),
        total_items: 1,
        total_pages: 1,
        items: vec![expected_song],
    };

    assert_eq!(response, expected_response);

    // ... but not on Premier DAM
    let response_empty = request.set_serial_no("AB316238").send();
    assert_eq!(response_empty.items.is_empty(), true);
}

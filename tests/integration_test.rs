// importing common module.
mod common;

#[tokio::test]
async fn test_get_albums() {
    common::setup();
    let album_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
    let album = shelving::get_album_by_spotify_uri(album_uri).await.unwrap();
    assert_eq!(album.name, "She's So Unusual");
}

mod common;

mod spotify {
    use rspotify::model::PlayableItem;


    #[tokio::test]
    async fn get_album_by_spotify_uri() {
        let spotify = crate::common::setup().await;

        let album_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
        let album = spotify.get_album_by_spotify_uri(album_uri).await.unwrap();
        assert_eq!(album.name, "She's So Unusual");
    }

    #[tokio::test]
    async fn get_currently_playing_track() {
        let spotify = crate::common::setup().await;
        let track = spotify.get_currently_playing_track().await;

        match track {
            None => assert!(true),
            Some(t) => assert_eq!(t.name, "Stockton"),
        }
    }
}

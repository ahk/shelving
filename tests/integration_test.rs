mod common;

mod spotify {
    use shelving::SpotifyApi;

    #[tokio::test]
    async fn get_album_by_spotify_uri() {
        crate::common::setup();
        let mut spotify: SpotifyApi = Default::default();
        spotify.setup_spotify_client().await;

        let album_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
        let album = spotify.get_album_by_spotify_uri(album_uri).await.unwrap();
        assert_eq!(album.name, "She's So Unusual");
    }
}

use shelving::SpotifyApi;

// Used in the examples: https://github.com/ramsayleung/rspotify/blob/master/Cargo.toml
#[tokio::main]
async fn main() {
    // TODO(ahk): should we just use the `new()` pattern here?
    let mut spotify: SpotifyApi = Default::default();
    spotify.setup_spotify_client().await;

    let album_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
    let albums = spotify.get_album_by_spotify_uri(album_uri).await;

    println!("Response: {:#?}", albums);
}

use shelving::SpotifyApi;

pub async fn setup() -> SpotifyApi {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
    let mut spotify: SpotifyApi = Default::default();
    spotify.setup_spotify_client().await;
    spotify
}

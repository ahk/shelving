use shelving::SpotifyApi;
use shelving::db::Db;

// Used in the examples: https://github.com/ramsayleung/rspotify/blob/master/Cargo.toml
#[tokio::main]
async fn main() {
    let mut db = Db();
    db.establish_connection();

    // TODO(ahk): should we just use the `new()` pattern here?
    let mut spotify = SpotifyApi::default();
    spotify.setup_spotify_client().await;

    let album_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
    let albums = spotify.get_album_by_spotify_uri(album_uri).await;

    println!("Response: {:#?}", albums);
}

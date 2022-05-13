use shelving::get_album_by_spotify_uri;

// Used in the examples: https://github.com/ramsayleung/rspotify/blob/master/Cargo.toml
#[tokio::main]
async fn main() {
    let album_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
    let albums = get_album_by_spotify_uri(album_uri).await;

    println!("Response: {:#?}", albums);
}

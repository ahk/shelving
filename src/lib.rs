use std::process::exit;
use rspotify::{model::AlbumId, prelude::*, ClientCredsSpotify, Credentials};

pub async fn get_album_by_spotify_uri(uri: &str) -> Result<rspotify::model::FullAlbum, rspotify::ClientError> {
    // You can use any logger for debugging.
    // env_logger::init();
    // Set RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET in an .env file or
    // export them manually:
    //
    // export RSPOTIFY_CLIENT_ID="your client_id"
    // export RSPOTIFY_CLIENT_SECRET="secret"
    // export RSPOTIFY_REDIRECT_URI="your redirect uri"
    //
    // These will then be read with `from_env`.
    //
    // Otherwise, set client_id and client_secret explictly:
    //
    // ```
    // let creds = Credentials {
    //     id: "this-is-my-client-id".to_string(),
    //     secret: Some("this-is-my-client-secret".to_string())
    // };
    // ```
    let creds = match Credentials::from_env() {
        None => {
            println!("Missing .env file.");
            exit(1);
        },
        Some(c) => c
    };

    let mut spotify = ClientCredsSpotify::new(creds);
    // Obtaining the access token. Requires to be mutable because the internal
    // token will be modified. We don't need OAuth for this specific endpoint,
    // so `...` is used instead of `prompt_for_user_token`.
    spotify.request_token().await.unwrap();

    // Running the requests
    let album_id = AlbumId::from_uri(uri).unwrap();
    let album = spotify.album(&album_id).await;
    album
}
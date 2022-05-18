use std::process::exit;
use rspotify::{model::AlbumId, prelude::*, ClientCredsSpotify, Credentials};

// NOTES:
// - Looking at the number of individual requests we'll need to make to check if a resource is saved in an account
// makes me think we'll need to really cut down on how much we mirror, or how often we mirror.
// If we're doing really long (maybe multiple days with a huge collection) scans across, how can we make that reliably
// incremental? Sort by last-refreshed and treat that as the refresh queue?
// How can we analyze the API limits and their relationship to a theoretical maximum library?
// Will we at some point for very large collections need to "archive" or otherwise implement
// some kind of LRU cache inside a given account on a given service? If this tool works very well and can make a player
// on a device play the resource ... (say Spotify and Apple Music clients are both installed on an Android phone),
// then maybe we don't actually need to mirror into? Only to pull from. We could even treat this like some kind of consuming
// activity where we remove new content after its pulled and merged with main collection to keep future pulls responsive.

#[derive(Clone, Debug, Default)]
pub struct SpotifyApi {
    is_ready: bool,
    spotify: ClientCredsSpotify
}

impl SpotifyApi {
    pub async fn setup_spotify_client(&mut self) {
        if self.is_ready {
            return;
        }

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

        // Requires to be mutable because the internal token will be modified.
        self.spotify = ClientCredsSpotify::new(creds);

        // Obtaining the access token.  We don't need OAuth for this specific endpoint,
        // so `...` is used instead of `prompt_for_user_token`.
        self.spotify.request_token().await.unwrap();

        self.is_ready = true;
    }

    pub async fn transfer_playback_to_device(&self, deviceId: &str) {
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-users-available-devices
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/transfer-a-users-playback
    }

    // pub async fn get_currently_playing_track(&self) -> Result<rspotify::model::FullTrack, rspotify::ClientError> {
    //     // https://developer.spotify.com/documentation/android/
    //     // On android it is possible to register a receiver to notify on "new track gets on top of the playing queue"
    //     // If this also exists for other services (or can be hacked) we may have a way to record playback...
    //     // Will this work even for remote playback on another device? It does usually have its state mirrored locally to the phone.
    //     // note you can POLL for currently playing using web-api:
    //     // https://developer.spotify.com/console/get-users-currently-playing-track/
    //     // https://developer.spotify.com/console/get-recently-played/

    //     // this is much harder for apple music, but must be available somewhere within (thankfully there's an Android SDK as well):
    //     // https://developer.apple.com/musickit/
    //     // Running the requests
    //     let track = self.spotify.current_playing(None).await;
    //     track
    // }

    pub async fn get_saved_albums(&self) {
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-saved-albums
        // this is paginated, max 50 per page
    }

    pub async fn save_albums(&self) {
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/save-albums-user
    }

    pub async fn remove_albums(&self) {
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-albums-user
    }

    pub async fn check_saved_albums(&self) {
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/check-users-saved-albums
    }

    pub async fn get_album_by_spotify_uri(&self, uri: &str) -> Result<rspotify::model::FullAlbum, rspotify::ClientError> {
        // Running the requests
        let album_id = AlbumId::from_uri(uri).unwrap();
        let album = self.spotify.album(&album_id).await;
        album
    }
}

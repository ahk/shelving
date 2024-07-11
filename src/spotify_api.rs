use async_std::io::empty;
// use rspotify::{model::AlbumId, prelude::*, ClientCredsSpotify, Credentials, SpotifyOAuth};
use rspotify::{
    model::{AlbumId, FullTrack, Market, PlayableItem}, prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth
};

#[derive(Clone, Debug, Default)]
pub struct SpotifyApi {
    is_ready: bool,
    spotify: AuthCodeSpotify,
    currently_playing_track_queue: Vec<FullTrack>,
    last_played_track: Option<FullTrack>,
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

        // let creds = match Credentials::from_env() {
        //     None => {
        //         println!("Missing .env file.");
        //         exit(1);
        //     },
        //     Some(c) => c
        // };

        // // Requires to be mutable because the internal token will be modified.
        // self.spotify = ClientCredsSpotify::new(creds);

        // // Obtaining the access token.  We don't need OAuth for this specific endpoint,
        // // so `...` is used instead of `prompt_for_user_token`.
        // self.spotify.request_token().await.unwrap();

        // The credentials must be available in the environment. Enable
        // `env-file` in order to read them from an `.env` file.
        let creds = Credentials::from_env().unwrap();

        // Using every possible scope
        let scopes = scopes!(
            "user-read-email",
            "user-read-private",
            "user-top-read",
            "user-read-recently-played",
            "user-follow-read",
            "user-library-read",
            "user-read-currently-playing",
            "user-read-playback-state",
            "user-read-playback-position",
            "playlist-read-collaborative",
            "playlist-read-private",
            "user-follow-modify",
            "user-library-modify",
            "user-modify-playback-state",
            "playlist-modify-public",
            "playlist-modify-private",
            "ugc-image-upload"
        );
        let oauth = OAuth::from_env(scopes).unwrap();

        let config = Config {
            token_cached: true,
            token_refreshing: true,
            ..Default::default()
        };

        let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

        let url = spotify.get_authorize_url(false).unwrap();
        spotify.prompt_for_token(&url).await.unwrap();

        // {
        //     let token = spotify.token.lock().await.unwrap();
        //     println!("Access token: {}", &token.as_ref().unwrap().access_token);
        //     println!(
        //         "Refresh token: {}",
        //         token.as_ref().unwrap().refresh_token.as_ref().unwrap()
        //     );
        // }

        self.spotify = spotify;

        self.is_ready = true;
    }

    pub async fn transfer_playback_to_device(&self, _device_id: &str) {
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-users-available-devices
        // https://developer.spotify.com/documentation/web-api/reference/#/operations/transfer-a-users-playback
    }

    pub async fn get_currently_playing_track(&self) -> Option<FullTrack> {
        // https://developer.spotify.com/documentation/android/
        // On android it is possible to register a receiver to notify on "new track gets on top of the playing queue"
        // If this also exists for other services (or can be hacked) we may have a way to record playback...
        // Will this work even for remote playback on another device? It does usually have its state mirrored locally to the phone.
        // note you can POLL for currently playing using web-api:
        // https://developer.spotify.com/console/get-users-currently-playing-track/
        // https://developer.spotify.com/console/get-recently-played/

        // this is much harder for apple music, but must be available somewhere within (thankfully there's an Android SDK as well):
        // https://developer.apple.com/musickit/
        // Running the requests
        // FIXME(ahk): I literally do not understand what `None, None::<&[_]>` means and why there isn't a more sensible way to do empty/optional args
        let playing_context = self.spotify.current_playing(None, None::<&[_]>).await;
        let context = match playing_context {
            Ok(context) => {
                context
            },
            Err(err) => {
                let msg = err.to_string();
                println!("Err trying {msg}");
                None
            }
        };

        let playable = match context {
            None => None,
            Some(ctx) => match ctx.item {
                None => None,
                Some(playable) => Some(playable)
            }
        };

        let track = match playable {
            None => None,
            Some(PlayableItem::Episode(_)) => None,
            Some(PlayableItem::Track(t)) => Some(t)
        };

        track
    }

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
        let album = self.spotify.album(album_id, None).await;
        album
    }

    // Append our latest current track to the play history if it doesn't match the last played track
    pub fn process_currently_playing_track(&mut self, opt_track: Option<FullTrack>) {
        if let Some(track) = &opt_track  {
            let is_new_track = match &self.last_played_track {
                // It's new if we don't have a last track
                None => true,

                Some(last_track) => {
                    if last_track.name != track.name {
                        // It's new if the previously played track is different
                        true
                    } else {
                        // It's old if we're still playing the same track
                        false
                    }
                }
            };

            if is_new_track {
                self.currently_playing_track_queue.push(track.clone());
            }
        } else {
            // TODO(ahk) We have played at least one track but are now playing nothing
            // might want to update something on the last track in currently playing queue?
        }

        // We update our last track status even when None, because that also will indicate a new track
        self.last_played_track = opt_track;
    }

    pub fn history_currently_playing(&self) -> &Vec<FullTrack> {
        return &self.currently_playing_track_queue;
    }

    // Occasionally need to consume the queue, but make sure we leave the latest track
    pub fn drain_current_playing(&mut self) -> Vec<FullTrack> {
        let tlen = self.currently_playing_track_queue.len();
        let played_tracks = self.currently_playing_track_queue
            // can this be done better with a swap?
            .drain(0..tlen)
            .collect();
        return played_tracks;
    }
}

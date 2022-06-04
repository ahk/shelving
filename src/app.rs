use std::time::Duration;
use async_std::task::sleep;

use crate::db::Db;
use crate::spotify_api::SpotifyApi;

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

pub struct App {
    db: Db,
    spotify_api: SpotifyApi,
}

impl App {
    pub async fn init() -> Self {
        let mut app = App {
            db: Db::new(),
            // TODO(ahk): should we just use the `new()` pattern here?
            spotify_api: SpotifyApi::default()
        };
        app.db.establish_connection();
        app.spotify_api.setup_spotify_client().await;

        app
    }

    pub async fn run(&mut self) {
        let playback_history = self.db.get_track_plays();
        println!("playback history: ");
        println!("{:?}", playback_history);

        loop {
            let opt_track = self.spotify_api.get_currently_playing_track().await;
            println!("currently playing: {:?}", opt_track);

            self.spotify_api.process_currently_playing_track(opt_track);
            db.
            
            sleep(Duration::from_secs(10)).await;
        }

        // let album_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
        // let albums = spotify.get_album_by_spotify_uri(album_uri).await;
        // println!("Response: {:#?}", albums);
    }
}
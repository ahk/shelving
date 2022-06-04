mod common;

mod spotify {
    use rspotify::model::{FullTrack};

    const FIXTURE_CURRENT_PLAYING_TRACK: &str = r#"
        {"album":{"album_type":"album","artists":[{"external_urls":{"spotify":"https://open.spotify.com/artist/5RADpgYLOuS2ZxDq7ggYYH"},"href":"https://api.spotify.com/v1/artists/5RADpgYLOuS2ZxDq7ggYYH","id":"5RADpgYLOuS2ZxDq7ggYYH","name":"Death Grips"}],"available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BE","BF","BG","BH","BI","BJ","BN","BO","BR","BS","BT","BW","BY","BZ","CA","CD","CG","CH","CI","CL","CM","CO","CR","CV","CW","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HU","ID","IE","IL","IN","IQ","IS","IT","JM","JO","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","LY","MA","MC","MD","ME","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RS","RW","SA","SB","SC","SE","SG","SI","SK","SL","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","US","UY","UZ","VC","VE","VN","WS","XK","ZA","ZM","ZW"],"external_urls":{"spotify":"https://open.spotify.com/album/08aqY8lv4zx4uaqBUpMD8a"},"href":"https://api.spotify.com/v1/albums/08aqY8lv4zx4uaqBUpMD8a","id":"08aqY8lv4zx4uaqBUpMD8a","images":[{"height":640,"url":"https://i.scdn.co/image/ab67616d0000b273f552daab2bc3dc64d2c4c649","width":640},{"height":300,"url":"https://i.scdn.co/image/ab67616d00001e02f552daab2bc3dc64d2c4c649","width":300},{"height":64,"url":"https://i.scdn.co/image/ab67616d00004851f552daab2bc3dc64d2c4c649","width":64}],"name":"No Love Deep Web","release_date":"2012-10-01","release_date_precision":"day"},"artists":[{"external_urls":{"spotify":"https://open.spotify.com/artist/5RADpgYLOuS2ZxDq7ggYYH"},"href":"https://api.spotify.com/v1/artists/5RADpgYLOuS2ZxDq7ggYYH","id":"5RADpgYLOuS2ZxDq7ggYYH","name":"Death Grips"}],"available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BE","BF","BG","BH","BI","BJ","BN","BO","BR","BS","BT","BW","BY","BZ","CA","CD","CG","CH","CI","CL","CM","CO","CR","CV","CW","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HU","ID","IE","IL","IN","IQ","IS","IT","JM","JO","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","LY","MA","MC","MD","ME","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RS","RW","SA","SB","SC","SE","SG","SI","SK","SL","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","US","UY","UZ","VC","VE","VN","WS","XK","ZA","ZM","ZW"],"disc_number":1,"duration_ms":197266,"explicit":true,"external_ids":{"isrc":"USUG11300642"},"external_urls":{"spotify":"https://open.spotify.com/track/6myqxcSQ85dyxg4XzTzzEL"},"href":"https://api.spotify.com/v1/tracks/6myqxcSQ85dyxg4XzTzzEL","id":"6myqxcSQ85dyxg4XzTzzEL","is_local":false,"name":"Stockton","popularity":34,"preview_url":"https://p.scdn.co/mp3-preview/5c72d3212b7e181988d9ee6f36cea7b875ebb581?cid=07220da624524664823bee614467b3bc","track_number":10}
    "#;

    #[tokio::test]
    async fn get_album_by_spotify_uri() {
        let spotify = crate::common::setup().await;

        let album_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
        let album = spotify.get_album_by_spotify_uri(album_uri).await.unwrap();
        assert_eq!(album.name, "She's So Unusual");
    }

    // FIXME(ahk): This only works if you play nothing or the song "Stockton" on your actual account when the test runs
    #[tokio::test]
    async fn get_currently_playing_track() {
        let spotify = crate::common::setup().await;
        let track = spotify.get_currently_playing_track().await;

        // TODO: Use something like this to log the json from an API call ... can we automatically cache this once
        // and allow for incremental updating of APIs with occasional real test against backend but mostly use cache?
        // let log_track = serde_json::to_string(&track).unwrap();
        // println!("{log_track}");

        match track {
            None => assert!(true),
            Some(t) => assert_eq!(t.name, "Stockton"),
        }
    }

    #[tokio::test]
    async fn store_currently_playing_track() {
        let mut spotify = crate::common::setup().await;
        
        let current = serde_json::from_str::<FullTrack>(FIXTURE_CURRENT_PLAYING_TRACK).unwrap();
        let exp_name = current.name.clone();

        spotify.process_currently_playing_track(Some(current));
        let h_tracks = spotify.history_currently_playing();
        let h_track = h_tracks.first().unwrap();

        assert_eq!(exp_name, h_track.name);
    }
}

use rspotify::model::FullTrack;
use crate::schema::{track, track_play};

#[derive(Queryable, Clone, Debug, Insertable)]
// FIXME(ahk): accursed auto-pluralization for table names is used, so we override
#[table_name = "track"]
pub struct Track {
    pub id: Option<i32>,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Clone, Debug, Insertable)]
#[table_name = "track_play"]
pub struct TrackPlay {
    pub id: Option<i32>,
    pub track_id: i32,
    pub created_at: String,
}

impl Track {
    pub fn for_spotify_track(ft: &FullTrack) -> Self {
        Track {
            id: None, 
            name: ft.name.clone(),
            // FIXME(ahk): how to handle DEFAULT timestamp values for sqlite in this godforesaken ORM
            updated_at: String::new(),
            created_at: String::new(),
        }
    }
}

impl TrackPlay {
    // TODO(ahk): maybe there is something nice to do here using relations in diesel?
    pub fn for_track(t: &Track) -> Self {
        TrackPlay {
            id: None,
            // FIXME(ahk): can we find a way to prevent callers from sending unsaved records to this function?
            // Should we separate them so saved records have non-optional id in their type?
            track_id: t.id
                .expect("for_track: Can't use unsaved records to create TrackPlay"),
            // FIXME(ahk): how to handle DEFAULT timestamp values for sqlite in this godforesaken ORM
            created_at: String::new()
        }
    }
}
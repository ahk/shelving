use diesel::dsl::sql;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::SqliteConnection;
use std::env;

use crate::schema;
use crate::models::*;

pub struct Db {
    connection: Option<SqliteConnection>
}

impl Db {
    pub fn new() -> Self {
        return Db { connection: None };
    }

    pub fn get_conn(&self) -> &SqliteConnection {
        return self.connection.as_ref().unwrap()
    }

    pub fn establish_connection(&mut self) -> bool {
        if self.connection.is_some() {
            return true;
        }

        dotenv().ok();
        // let pbuf = env::current_dir().unwrap();
        // println!("where: {:?}", pbuf);
        
        // println!("how many: dote {} e {}", dotenv::vars().count(), env::vars().count());

        let envs: Vec<String> = env::vars().map(|v| {
            println!("env-item: {} {}", v.0, v.1);
            v.0.clone()
        })
        .collect();
        println!("what else: {:?}", envs);

        // let database_url_env = env::var("DATABASE_URL")
        //     .expect("DATABASE_URL must be set");

        let database_url = "db/shelving.sqlite3";

        self.connection = match SqliteConnection::establish(&database_url) {
            Ok(conn) => Some(conn),
            Err(err) => {
                println!("establish_connection: Error {}", err);
                None
            }
        };

        return self.connection.is_some();
    }

    // FIXME(ahk): DRY this with generic args?
    pub fn add_tracks(&self, tracks: &Vec<Track>) -> Option<Vec<Track>> {
        let lname = "add_tracks";
        use schema::track::dsl::*;

        if self.connection.is_none() {
            println!("{lname}: Error: no connection available");
            return None;
        }

        let mut rows_inserted : Vec<Track> = vec![];
        for t in tracks {
            diesel::insert_into(track)
                .values(t)
                .execute(self.get_conn())
                .expect("{lname}: Error: failed to insert track rows");
            // FIXME(ahk): Because Diesel is currently (?) still horrible for sqlite, they haven't enabled the ability to
            // get any columns of a record returned from an insert. So we have to do whatever this stupid thing is,
            // one by one, for each row.
            // https://github.com/diesel-rs/diesel/discussions/2684
            let result: Result<Track, diesel::result::Error> = track
                .find(sql("last_insert_rowid()"))
                .get_result(self.get_conn());

            match result {
                Ok(inserted) => {
                    rows_inserted.push(inserted)
                },
                Err(err) => {
                    println!("{lname}: Error finding last inserted track - {:?}", err)
                }
            }
        }
       
        return Some(rows_inserted);
    }

    pub fn add_track_plays(&self, plays: &Vec<TrackPlay>) -> Option<Vec<TrackPlay>> {
        let lname = "add_track_plays";
        use schema::track_play::dsl::*;

        if self.connection.is_none() {
            println!("{lname}: Error: no connection available");
            return None;
        }

        let mut rows_inserted : Vec<TrackPlay> = vec![];
        for play in plays {
            diesel::insert_into(track_play)
                .values(play)
                .execute(self.get_conn())
                .expect("{lname}: Error: failed to insert track_play rows");
            // FIXME(ahk): Because Diesel is currently (?) still horrible for sqlite, they haven't enabled the ability to
            // get any columns of a record returned from an insert. So we have to do whatever this stupid thing is,
            // one by one, for each row.
            // https://github.com/diesel-rs/diesel/discussions/2684
            let result: Result<TrackPlay, diesel::result::Error> = track_play
                .find(sql("last_insert_rowid()"))
                .get_result(self.get_conn());

            match result {
                Ok(inserted) => {
                    rows_inserted.push(inserted)
                },
                Err(err) => {
                    println!("{lname}: Error finding last inserted track_play - {:?}", err)
                }
            }
        }
       
        return Some(rows_inserted);
    }

    pub fn get_track_plays(&self) -> Option<Vec<TrackPlay>> {
        use schema::track_play::dsl::*;

        if self.connection.is_none() {
            return None;
        }
        
        let results = track_play.limit(5)
            .load::<TrackPlay>(self.get_conn())
            .expect("Error loading track plays");

        println!("Displaying {} track plays", results.len());

        for play in results.clone() {
            println!("{}", play.id.unwrap_or(0));
            println!("----------\n");
            println!("{}", play.track_id);
        }

        return Some(results);
    }

    pub fn get_track(&self, track_id: i32) -> Option<Track> {
        use schema::track::dsl::*;

        if self.connection.is_none() {
            return None;
        }

        let result = track
            .find(track_id)
            .first(self.get_conn());

        let t = match result {
            Ok(res) => Some(res),
            Err(err) => {
                println!("get_track: Error {}", err);
                None
            }
        };

        return t;
    }
}

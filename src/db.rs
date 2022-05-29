extern crate dotenv;

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

        // dotenv().ok();
        // let pbuf = env::current_dir().unwrap();
        // println!("where: {:?}", pbuf);
        
        // println!("how many: dote {} e {}", dotenv::vars().count(), env::vars().count());

        // let envs: Vec<String> = env::vars().map(|v| {
        //     v.0.clone()
        // })
        // .collect();
        // println!("what else: {:?}", envs);

        // let database_url = env::var("DATABASE_URL")
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

    pub fn get_track_plays(&self) {
        use schema::track_play::dsl::*;

        if self.connection.is_none() {
            return;
        }
        
        let results = track_play.limit(5)
            .load::<TrackPlay>(self.get_conn())
            .expect("Error loading track plays");

        println!("Displaying {} track plays", results.len());

        for play in results {
            println!("{}", play.id.unwrap_or(0));
            println!("----------\n");
            println!("{}", play.track_id);
        }
    }

    pub fn get_track(&self, track_id: i32) -> Option<Track> {
        use schema::track::dsl::*;

        if self.connection.is_none() {
            return None;
        }

        let result = track.find(track_id)
            .first::<Track>(self.get_conn());

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

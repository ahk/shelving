# This project uses the diesel CLI

## Info about the sqlite3 db and diesel

(the sorta-kinda-ORM)

### It can be installed with the following

```bash
cargo install diesel_cli --version '2.2.1' --no-default-features --features 'sqlite-bundled'
```

### Setup diesel in the project

Sets up directory skeleton, creates db, runs initial migrations. You should only need to do this once.

TODO(ahk): it's unclear why we need to specify this url as an arg when the doc for the tool says it uses `.env`
And if it doesn't, do we actually need it for the diesel lib?

```bash
diesel --database-url 'db/shelving.sqlite3' setup
```

### Creating a diesel migration

```bash
diesel --database-url 'db/shelving.sqlite3' migration generate create_track_plays_table
```

### Running a diesel migration

```
diesel --database-url 'db/shelving.sqlite3' migration run
```

### Running tests matching a given string with printing
```
cargo test spotify::get_album_by_spotify_uri -- --nocapture
```
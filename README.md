# This project uses the diesel CLI

## It can be installed with the following

```bash
cargo install diesel_cli --version '1.4.1' --no-default-features --features 'sqlite-bundled'
```

## Setup diesel in the project

Sets up directory skeleton, creates db, runs initial migrations

```bash
diesel --database-url 'db/shelving.sqlite3' setup
```

use shelving::app::App;

// Used in the examples: https://github.com/ramsayleung/rspotify/blob/master/Cargo.toml
#[tokio::main]
async fn main() {
    let mut app = App::init().await;
    app.run().await;
}

#[derive(Queryable)]
pub struct Track {
    pub id: Option<i32>,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable)]
pub struct TrackPlay {
    pub id: Option<i32>,
    pub track_id: i32,
    pub created_at: String,
}
table! {
    track (id) {
        id -> Nullable<Integer>,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    track_play (id) {
        id -> Nullable<Integer>,
        track_id -> Integer,
        created_at -> Timestamp,
    }
}

joinable!(track_play -> track (track_id));

allow_tables_to_appear_in_same_query!(
    track,
    track_play,
);

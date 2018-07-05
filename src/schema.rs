table! {
    repos (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
    }
}

table! {
    profiles (id) {
        id -> Integer,
        repo_id -> Integer,
        name -> Text,
    }
}

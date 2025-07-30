use diesel::{table, joinable, allow_tables_to_appear_in_same_query};

table! {
    origins (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    categories (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    violationtypes (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        dob -> Timestamp,
        image -> Varchar,
        role -> Varchar,
        origin_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_followings (user_id, followed_user_id) {
        user_id -> Uuid,
        followed_user_id -> Uuid,
    }
}

table!{
    comments (id) {
        id -> Uuid,
        message -> VarChar,
        created_at -> Timestamp,
        user_id -> Uuid
    }
}

table! {
    user_likes (user_id, comment_id) {
        user_id -> Uuid,
        comment_id -> Uuid,
    }
}

table!{
    posts (id) {
        id -> Uuid,
        image -> VarChar,
        description -> VarChar,
        view -> Int4,
        created_at -> Timestamp,
        user_id -> Uuid,
        category_id ->Uuid
    }
}

joinable!(users -> origins (origin_id));
joinable!(comments -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    origins,
);

allow_tables_to_appear_in_same_query!(
    users,
    user_followings,
);



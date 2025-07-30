// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    chats (id) {
        id -> Uuid,
        #[max_length = 255]
        message -> Varchar,
        toid -> Nullable<Uuid>,
        fromid -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    commentlikes (userid, commentid) {
        userid -> Uuid,
        commentid -> Uuid,
    }
}

diesel::table! {
    comments (id) {
        id -> Uuid,
        #[max_length = 255]
        message -> Varchar,
        userid -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    origins (id) {
        id -> Uuid,
        #[max_length = 255]
        country -> Varchar,
    }
}

diesel::table! {
    postcomments (postid, commentid) {
        postid -> Uuid,
        commentid -> Uuid,
    }
}

diesel::table! {
    postlikes (userid, postid) {
        userid -> Uuid,
        postid -> Uuid,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        #[max_length = 255]
        image -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        views -> Nullable<Int8>,
        categoryid -> Nullable<Uuid>,
        userid -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    replycomments (id) {
        id -> Uuid,
        #[max_length = 255]
        message -> Varchar,
        commentid -> Nullable<Uuid>,
        userid -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    reports (id) {
        id -> Uuid,
        #[max_length = 255]
        description -> Varchar,
        #[max_length = 255]
        image -> Varchar,
        violationtypeid -> Nullable<Uuid>,
        userid -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    songs (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        audio -> Varchar,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    userfollowings (userid, followeduserid) {
        userid -> Uuid,
        followeduserid -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        dob -> Nullable<Date>,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        role -> Varchar,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        originid -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    usersongs (userid, songid) {
        userid -> Uuid,
        songid -> Uuid,
    }
}

diesel::table! {
    violationtypes (id) {
        id -> Uuid,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Varchar,
    }
}

diesel::joinable!(commentlikes -> comments (commentid));
diesel::joinable!(commentlikes -> users (userid));
diesel::joinable!(comments -> users (userid));
diesel::joinable!(postcomments -> comments (commentid));
diesel::joinable!(postcomments -> posts (postid));
diesel::joinable!(postlikes -> posts (postid));
diesel::joinable!(postlikes -> users (userid));
diesel::joinable!(posts -> categories (categoryid));
diesel::joinable!(posts -> users (userid));
diesel::joinable!(replycomments -> comments (commentid));
diesel::joinable!(replycomments -> users (userid));
diesel::joinable!(reports -> users (userid));
diesel::joinable!(reports -> violationtypes (violationtypeid));
diesel::joinable!(users -> origins (originid));
diesel::joinable!(usersongs -> songs (songid));
diesel::joinable!(usersongs -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    chats,
    commentlikes,
    comments,
    origins,
    postcomments,
    postlikes,
    posts,
    replycomments,
    reports,
    songs,
    userfollowings,
    users,
    usersongs,
    violationtypes,
);

// @generated automatically by Diesel CLI.

diesel::table! {
    chats (id) {
        id -> Uuid,
        #[max_length = 255]
        message -> Varchar,
        toid -> Uuid,
        fromid -> Uuid,
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
        userid -> Uuid,
        created_at -> Timestamp,
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
    postfiles (postid, files) {
        postid -> Uuid,
        #[max_length = 255]
        files -> Varchar,
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
        description -> Varchar,
        views -> Int8,
        userid -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    replycomments (id) {
        id -> Uuid,
        #[max_length = 255]
        message -> Varchar,
        commentid -> Uuid,
        userid -> Uuid,
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
        violationtypeid -> Uuid,
        userid -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    requests (id) {
        id -> Uuid,
        #[max_length = 255]
        description -> Varchar,
        userid -> Uuid,
        requesttypeid -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    requesttypes (id) {
        id -> Uuid,
        #[max_length = 255]
        category -> Varchar,
    }
}

diesel::table! {
    resettokens (id) {
        id -> Uuid,
        userid -> Uuid,
        created_at -> Timestamp,
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
        created_at -> Timestamp,
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
        #[max_length = 20]
        gender -> Varchar,
        dob -> Nullable<Date>,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        bio -> Varchar,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        #[max_length = 255]
        role -> Varchar,
        isverified -> Bool,
        originid -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(commentlikes -> comments (commentid));
diesel::joinable!(commentlikes -> users (userid));
diesel::joinable!(comments -> users (userid));
diesel::joinable!(postcomments -> comments (commentid));
diesel::joinable!(postcomments -> posts (postid));
diesel::joinable!(postfiles -> posts (postid));
diesel::joinable!(postlikes -> posts (postid));
diesel::joinable!(postlikes -> users (userid));
diesel::joinable!(posts -> users (userid));
diesel::joinable!(replycomments -> comments (commentid));
diesel::joinable!(replycomments -> users (userid));
diesel::joinable!(reports -> users (userid));
diesel::joinable!(reports -> violationtypes (violationtypeid));
diesel::joinable!(requests -> requesttypes (requesttypeid));
diesel::joinable!(requests -> users (userid));
diesel::joinable!(resettokens -> users (userid));
diesel::joinable!(users -> origins (originid));
diesel::joinable!(usersongs -> songs (songid));
diesel::joinable!(usersongs -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    commentlikes,
    comments,
    origins,
    postcomments,
    postfiles,
    postlikes,
    posts,
    replycomments,
    reports,
    requests,
    requesttypes,
    resettokens,
    songs,
    userfollowings,
    users,
    usersongs,
    violationtypes,
);

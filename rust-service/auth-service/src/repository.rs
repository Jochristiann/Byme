use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;
use model::schema::resettokens::dsl::resettokens;
use model::users::{NewToken, NewUsers, ResetToken};
use model::schema::users::dsl::*;
use model::schema::resettokens::id as token_id;
use model::schema::resettokens::created_at as token_time;
use model::state::{DbPool};

pub async fn insert_user(pool: &DbPool, new_user: &NewUsers) -> bool {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let result = diesel::insert_into(users)
        .values(new_user)
        .execute(conn);
    
    result.is_ok()
}

pub async fn change_password(pool: &DbPool, ids: Uuid, new_pass: String) -> bool {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    diesel::update(users.filter(id.eq(ids)))
        .set(password.eq(new_pass))
        .execute(conn)
        .is_ok()
}

pub async fn insert_token(pool: &DbPool, new_token:NewToken) -> bool {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let result = diesel::insert_into(resettokens)
        .values(new_token)
        .execute(conn);

    result.is_ok()
}

pub async fn get_token(pool: &DbPool, ids: Uuid) -> Option<ResetToken> {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    resettokens
        .filter(token_id.eq(ids))
        .first::<ResetToken>(conn)
        .optional()
        .expect("Error loading token")
}

pub async fn delete_token(pool: &DbPool, token_ids: Uuid) -> bool {
    let conn = &mut pool.get().expect("Failed to get DB connection");


    let expiry_threshold: NaiveDateTime = (Utc::now() - Duration::minutes(5)).naive_utc();

    let _ = diesel::delete(resettokens.filter(token_time.lt(expiry_threshold)))
        .execute(conn)
        .is_ok();

    diesel::delete(resettokens.filter(token_id.eq(token_ids))).execute(conn).is_ok()


}

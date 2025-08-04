use diesel::prelude::*;
use uuid::Uuid;
use model::users::{NewUsers};
use model::schema::users::dsl::*;
use model::state::DbPool;

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

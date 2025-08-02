use diesel::prelude::*;
use uuid::Uuid;
use model::users::{RegisterUsers};
use model::schema::users::dsl::*;
use model::state::DbPool;

pub async fn insert_user(pool: &DbPool, new_user: &RegisterUsers) -> bool {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    diesel::insert_into(users)
        .values(new_user)
        .execute(conn)
        .is_ok()
}

pub async fn change_password(pool: &DbPool, ids: String, new_pass: String) -> bool {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    diesel::update(users.filter(id.eq(Uuid::parse_str(&ids).unwrap())))
        .set(password.eq(new_pass))
        .execute(conn)
        .is_ok()
}

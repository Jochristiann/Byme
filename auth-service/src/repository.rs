use diesel::prelude::*;
use model::users::{RegisterUsers, Users};
use model::schema::users::dsl::*;
use model::state::DbPool;

pub async fn insert_user(pool: &DbPool, new_user: &RegisterUsers) -> bool {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    diesel::insert_into(users)
        .values(new_user)
        .execute(conn)
        .is_ok()
}

pub async fn get_user_by_email(pool: &DbPool, email_input: String) -> Option<Users> {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    users
        .filter(email.eq(email_input))
        .first::<Users>(conn)
        .optional()
        .expect("Error loading user")
}

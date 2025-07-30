use diesel::prelude::*;
use model::users::{Users};
use model::schema::users::dsl::*;
use model::state::DbPool;

pub async fn insert_user(pool: &DbPool, new_user: &Users) -> bool {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    diesel::insert_into(users)
        .values(new_user)
        .execute(conn)
        .is_ok()
}

// pub async fn get_user_by_email(conn: &mut PgConnection, email_input: &str) -> Option<Users> {
//
//
//     // users
//     //     .filter(email.eq(email_input))
//     //     .first::<Users>(conn)
//     //     .ok()
//
// }

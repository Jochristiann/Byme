use diesel::prelude::*;
use crate::{model::users::{Users}, AppState};
use crate::model::schema::users::dsl::*;
pub async fn insert_user(new_user: &Users, state: &AppState) {
    let conn = &mut state.db.get("").unwrap();

    diesel::insert_into(users)
        .values(new_user)
        .execute(conn)
        .unwrap();
}

pub async fn get_user_by_email(email_input: &str, state: &AppState) -> Option<Users> {
    
    let conn = &mut state.db.get().unwrap();

    users
        .filter(email.eq(email_input))
        .first::<Users>(conn)
        .ok()
}

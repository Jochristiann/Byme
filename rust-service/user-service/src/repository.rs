use diesel::prelude::*;
use uuid::Uuid;
use model::master::Origins;
use model::schema::{origins, users};
use model::schema::users::dsl::users as users_dsl;
use model::schema::origins::dsl::{origins as origin_dsl};
use model::state::DbPool;
use model::users::{ Users};

pub async fn get_user_by_email(pool: &DbPool, email_input: String) -> Option<(Users,Option<Origins>)> {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let result = users_dsl
        .left_join(origin_dsl.on(users::originid.eq(origins::id.nullable())))
        .filter(users::email.eq(email_input))
        .first::<(Users, Option<Origins>)>(conn);

    result.ok()
}

pub async fn get_user_by_id(pool: &DbPool, id_input: Uuid) -> Option<(Users,Option<Origins>)> {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let result = users_dsl
        .left_join(origin_dsl.on(users::originid.eq(origins::id.nullable())))
        .filter(users::id.eq(id_input))
        .first::<(Users, Option<Origins>)>(conn);

    result.ok()
}

pub async fn get_user_by_role(pool: &DbPool, role_input: String) -> Option<Vec<(Users,Origins)>> {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let result = users_dsl
        .inner_join(origin_dsl.on(users::originid.eq(origins::id.nullable())))
        .filter(users::role.eq(role_input))
        .load::<(Users,Origins)>(conn)
        .ok();

    result
}

pub async fn get_user_by_username(pool: &DbPool, usn: String) -> Option<(Users,Option<Origins>)> {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let result = users_dsl
        .left_join(origin_dsl.on(users::originid.eq(origins::id.nullable())))
        .filter(users::username.eq(usn))
        .first::<(Users, Option<Origins>)>(conn);

    result.ok()
}
use uuid::Uuid;
use diesel::prelude::*;
use model::posts::{NewPost, Posts};
use model::schema::posts::dsl::posts as posts_dsl;
use model::schema::{ users};
use model::schema::users::dsl::users as users_dsl;
use model::schema::posts::{ id, userid};
use model::state::DbPool;
use model::users::Users;

pub async fn get_post_by_id(pool: &DbPool, ids:Uuid) ->Option<(Posts,Users)>{
    let conn = &mut pool.get().expect("Failed to get DB connection");

    posts_dsl
        .inner_join(users_dsl.on(users::id.eq(userid)))
        .filter(id.eq(ids))
        .first::<(Posts,Users)>(conn)
        .optional()
        .expect("Failed to get post by id")
}

pub async fn get_all_posts_by_user_id(pool: &DbPool, ids: Uuid)->Option<Vec<(Posts,Users)>>{
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let result = posts_dsl
        .inner_join(users_dsl.on(users::id.eq(userid)))
        .filter(userid.eq(ids))
        .load::<(Posts,Users)>(conn);

    result.ok()
}

pub async fn create_post(pool:&DbPool, post: NewPost) -> bool{
    let conn = &mut pool.get().expect("Failed to get DB connection");

    diesel::insert_into(posts_dsl).values(&post).execute(conn).is_ok()
}

pub async fn delete_post(pool:&DbPool, ids:Uuid) -> bool{
    let conn = &mut pool.get().expect("Failed to get DB connection");

    diesel::delete(posts_dsl).filter(id.eq(ids)).execute(conn).is_ok()
}
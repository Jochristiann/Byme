use uuid::Uuid;
use diesel::prelude::*;
use model::master::Categories;
use model::posts::{NewPost, Posts};
use model::schema::posts::dsl::posts as posts_dsl;
use model::schema::{categories, users};
use model::schema::users::dsl::users as users_dsl;
use model::schema::categories::dsl::{categories as categories_dsl};
use model::schema::categories::name;
use model::schema::posts::{categoryid, id, userid};
use model::state::DbPool;
use model::users::Users;

pub async fn get_post_by_id(pool: &DbPool, ids:Uuid) ->Option<(Posts,Users,Categories)>{
    let conn = &mut pool.get().expect("Failed to get DB connection");

    posts_dsl
        .inner_join(users_dsl.on(users::id.eq(userid)))
        .inner_join(categories_dsl.on(categories::id.eq(categoryid)))
        .filter(id.eq(ids))
        .first::<(Posts,Users,Categories)>(conn)
        .optional()
        .expect("Failed to get post by id")
}

pub async fn get_all_posts_by_user_id(pool: &DbPool, ids: Uuid)->Option<Vec<(Posts,Users,Categories)>>{
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let result = posts_dsl
        .inner_join(users_dsl.on(users::id.eq(userid)))
        .inner_join(categories_dsl.on(categories::id.eq(categoryid)))
        .filter(userid.eq(ids))
        .load::<(Posts,Users,Categories)>(conn);

    result.ok()
}

pub async fn get_all_posts_by_category(pool: &DbPool, category: String)->Option<Vec<(Categories,Posts,Users)>>{
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let result = categories_dsl
        .inner_join(posts_dsl.on(id.eq(userid)))
        .inner_join(users_dsl.on(users::id.eq(id)))
        .filter(name.eq(category))
        .load::<(Categories,Posts,Users)>(conn);

    result.ok()
}

pub async fn create_post(pool:&DbPool, post: NewPost) -> bool{
    let conn = &mut pool.get().expect("Failed to get DB connection");

    diesel::insert_into(posts_dsl).values(&post).execute(conn).is_ok()
}
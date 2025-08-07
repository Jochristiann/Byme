use diesel::{ExpressionMethods, RunQueryDsl};
use uuid::Uuid;
use model::comments::Comments;
use model::schema::comments::dsl::comments as comments_dsl;
use model::schema::comments::id;
use model::schema::posts::dsl::posts as posts_dsl;
use model::state::DbPool;

pub async fn delete_several_comments(pool:&DbPool, comment_id:Vec<Uuid>) -> bool{
    let conn = &mut pool.get().expect("Failed to get DB connection");
    diesel::delete(comments_dsl).filter(id.eq_any(comment_id)).execute(conn).is_ok()
}

pub async fn create_comment(pool:&DbPool, comment:Comments) -> bool {
    let conn = &mut pool.get().expect("Failed to get DB connection");
    diesel::insert_into(comments_dsl).values(&comment).execute(conn).is_ok()

}
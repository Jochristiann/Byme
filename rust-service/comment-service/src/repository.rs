use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use uuid::Uuid;
use model::comments::{Comments, NewComment};
use model::posts::PostComments;
use model::schema::comments::dsl::comments as comments_dsl;
use model::schema::comments::id;
use model::schema::postcomments::dsl::postcomments as postcomments_dsl;
use model::schema::postcomments::postid;
use model::state::DbPool;

pub async fn delete_several_comments(pool:&DbPool, comment_id:Vec<Uuid>) -> bool{
    let conn = &mut pool.get().expect("Failed to get DB connection");
    diesel::delete(comments_dsl).filter(id.eq_any(comment_id)).execute(conn).is_ok()
}

pub async fn delete_comment(pool:&DbPool, comment_id:Uuid) -> bool{
    let conn = &mut pool.get().expect("Failed to get DB connection");
    diesel::delete(comments_dsl).filter(id.eq(comment_id)).execute(conn).is_ok()
}

pub async fn create_comment(pool:&DbPool, comment:NewComment, post_comments: PostComments) -> bool {
    let conn = &mut pool.get().expect("Failed to get DB connection");
    let query1 = diesel::insert_into(comments_dsl).values(&comment).execute(conn).is_ok();
    let query2 = diesel::insert_into(postcomments_dsl).values(&post_comments).execute(conn).is_ok();
    query1 && query2
}

pub async fn get_comments_by_post_id (pool:&DbPool, post_id:Uuid) -> Option<Vec<(PostComments, Comments)>>{
    let conn = &mut pool.get().expect("Failed to get DB connection");
    postcomments_dsl
        .inner_join(comments_dsl.on(id.eq(post_id)))
        .filter(postid.eq(post_id))
        .load::<(PostComments,Comments)>(conn)
        .ok()
}

pub async fn get_comment_by_comment_id (pool:&DbPool, comment_id:Uuid) -> Option<Comments> {
    let conn = &mut pool.get().expect("Failed to get DB connection");
    comments_dsl
        .filter(id.eq(comment_id))
        .first::<Comments>(conn)
        .ok()
}
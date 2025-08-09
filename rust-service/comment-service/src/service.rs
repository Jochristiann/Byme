use uuid::Uuid;
use model::accessible::parse_id;
use model::comments::{CommentRequest, Comments, NewComment};
use model::posts::{PostComments};
use model::state::DbPool;
use crate::repository;


pub async fn get_all_comments_by_post_id(pool:&DbPool, post_id:String) -> Option<Vec<Comments>> {
    let parsed_id = parse_id(post_id);
    let responses = repository::get_comments_by_post_id(pool,parsed_id).await;

    if let Some(response) = responses {
        let mut comments = Vec::new();
        for (_, comment) in response {
            comments.push(comment);
        }
        return Some(comments)
    }

    None
}

pub async fn create_comment(pool:&DbPool, comment:CommentRequest, user_id: String ) -> bool {
    let comment_id = Uuid::new_v4();
    let parsed_user_id = parse_id(user_id);

    let new_comment = NewComment{
        id:comment_id.clone(),
        message: comment.message.to_string(),
        userid: parsed_user_id,
    };
    let parsed_post_id = parse_id(comment.postid);

    let post_comment = PostComments{
        postid: comment_id,
        commentid: parsed_post_id,
    };

    let response = repository::create_comment(pool,new_comment,post_comment).await;

    response
}

pub async fn delete_comment(pool:&DbPool, comment_id:String) -> bool {
    let parsed_id = parse_id(comment_id);
    let response = repository::delete_comment(pool, parsed_id).await;
    response
}
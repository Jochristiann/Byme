use model::comments::CommentRequest;
use model::state::DbPool;
use crate::service;

pub async fn create_comment(state: &DbPool, comment:CommentRequest, user_id: String) -> bool {
    let response = service::create_comment(state, comment, user_id).await;
    response
}

pub async fn delete_comment(state: &DbPool, comment_id:String) -> bool {
    let actual_user_id = service::get_user_id_by_comment(state,comment_id.clone()).await;
    if let Some(user_id) = actual_user_id {
        if user_id != comment_id.clone() {
            return false
        }
        let response = service::delete_comment(state, comment_id).await;
        return response
    }

    false
}
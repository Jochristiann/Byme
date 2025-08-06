use uuid::Uuid;
use model::accessible::parse_id;
use model::master::Categories;
use model::posts::{NewPost, PostRequest, PostResponse, Posts};
use model::state::DbPool;
use model::usable::save_base64_image_to_folder;
use model::users::{UserResponse, Users};
use user_service::service::convert_user_to_user_res;
use crate::repository;

pub async fn create_post(pool:&DbPool, post: PostRequest) -> bool{
    let new_uuid = Uuid::new_v4();
    let mut image_path = "".to_string();

    if !post.image.is_empty() {
        let folder = "";
        image_path = save_base64_image_to_folder(post.image.as_str(), folder, new_uuid.clone());
    }
 
    let cat_parsed = parse_id(post.category_id);
    let user_parsed = parse_id(post.user_id);

    let new_post = NewPost{
        id: new_uuid,
        image: image_path,
        description: post.description,
        categoryid: cat_parsed,
        userid: user_parsed,
    };

    let response = repository::create_post(pool,new_post).await;

    response
}

pub async fn delete_post(pool:&DbPool, ids:String) -> bool{
    let parsed_id = parse_id(ids);
    let response = repository::delete_post(pool,parsed_id ).await;

    response
}

pub async fn get_all_post_by_user_id(pool:&DbPool, user_id:String) ->Option<Vec<PostResponse>>{
    let parsed_id = parse_id(user_id);

    let responses = repository::get_all_posts_by_user_id(pool, parsed_id).await;
    if let Some(responses) = responses {
        let vec_converted = vec_convert_post_to_post_res(responses);
        return Some(vec_converted);
    }

    None
}

pub async fn get_all_post_by_category(pool:&DbPool, category:String) ->Option<Vec<PostResponse>>{

    let responses = repository::get_all_posts_by_category(pool, category).await;
    if let Some(responses) = responses {
        let mut converted_responses:Vec<PostResponse> = Vec::new();
        for (category,post, user) in responses {
            let user_response: UserResponse = convert_user_to_user_res(user, None);
            let post_response: PostResponse = convert_post_to_post_res(post, user_response, category);
            converted_responses.push(post_response);
        }
        return Some(converted_responses);
    }

    None
}

pub async fn get_post_by_id(pool:&DbPool, id:String) ->Option<PostResponse>{
    let parsed_id = parse_id(id);

    let responses = repository::get_post_by_id(pool, parsed_id).await;
    if let Some((post, user, category)) = responses {
        let user_response: UserResponse = convert_user_to_user_res(user, None);
        let post_response: PostResponse = convert_post_to_post_res(post, user_response, category);
        return Some(post_response);
    }

    None
}

pub fn vec_convert_post_to_post_res(posts: Vec<(Posts, Users, Categories)>)->Vec<PostResponse>{
    let mut converted_responses:Vec<PostResponse> = Vec::new();
    for (post, user, category) in posts {
        let user_response: UserResponse = convert_user_to_user_res(user, None);
        let post_response: PostResponse = convert_post_to_post_res(post, user_response, category);
        converted_responses.push(post_response);
    }
    converted_responses
}

pub fn convert_post_to_post_res(post:Posts, user:UserResponse, categories: Categories)->PostResponse{
    PostResponse{
        id: post.id,
        image: post.image,
        description: post.description,
        user,
        category:categories,
        views: post.views,
        created_at: post.created_at,
    }
}
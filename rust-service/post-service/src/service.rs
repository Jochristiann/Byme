use chrono::Utc;
use uuid::Uuid;
use model::accessible::parse_id;
use model::posts::{NewPost, PostFiles, PostRequest, PostResponse, Posts};
use model::state::DbPool;
use model::users::{UserResponse, Users};
use user_service::service::convert_user_to_user_res;
use crate::repository;

fn get_supabase_config() -> (String, String, String) {
    let supabase_url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_post_bucket = std::env::var("SUPABASE_POST_BUCKET").expect("SUPABASE_BUCKET must be set");
    let supabase_key = std::env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set");
    (supabase_url, supabase_post_bucket, supabase_key)
}

async fn upload_to_supabase(file_name: String, data: Vec<u8>) -> Result<String, String> {
    let (base_url, post_bucket, key) = get_supabase_config();
    let url = format!("{}/storage/v1/object/{}/{}", base_url, post_bucket, file_name);

    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", key))
        .header("Content-Type", "application/octet-stream")
        .body(data)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        Ok(format!("{}/storage/v1/object/public/{}/{}", base_url, post_bucket, file_name))
    } else {
        Err(format!("Failed to upload to Supabase: {}", res.status()))
    }
}

fn generate_unique_filename(original_name: &str) -> String {
    let ext = original_name
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase();

    let timestamp = Utc::now().timestamp_millis();
    let id = Uuid::new_v4();

    if ext.is_empty() {
        format!("{}-{}", id, timestamp)
    } else {
        format!("{}-{}.{}", id, timestamp, ext)
    }
}
pub async fn create_post(pool: &DbPool, post: PostRequest, user_id: String) -> bool {
    let new_post_id = Uuid::new_v4();
    let user_parsed = parse_id(user_id);

    let new_post = NewPost {
        id: new_post_id,
        description: post.description.clone(),
        userid: user_parsed,
    };

    if !repository::create_post(pool, new_post).await {
        return false;
    }

    for (original_name, file_data) in post.files {
        let unique_name = generate_unique_filename(&original_name);

        let public_url = match upload_to_supabase(unique_name, file_data).await {
            Ok(url) => url,
            Err(_) => return false,
        };

        let post_file = PostFiles {
            postid: new_post_id,
            files: public_url,
        };

        if !repository::insert_post_file(pool, post_file).await {
            return false;
        }
    }

    true
}

pub async fn delete_post(pool:&DbPool, ids:String) -> bool{
    let parsed_id = parse_id(ids);
    let response = repository::delete_post(pool,parsed_id).await;

    response
}

pub async fn get_all_post_by_user_id(pool:&DbPool, user_id:String) ->Option<Vec<PostResponse>>{
    let parsed_id = parse_id(user_id);

    let responses = repository::get_all_posts_by_user_id(pool, parsed_id).await;
    if let Some(responses) = responses {
        let vec_converted = vec_convert_post_to_post_res(pool,responses).await;
        return Some(vec_converted);
    }

    None
}

pub async fn get_post_by_id(pool:&DbPool, id:String) ->Option<(PostResponse, String)>{
    let parsed_id = parse_id(id);

    let responses = repository::get_post_by_id(pool, parsed_id).await;

    if let Some((post, user)) = responses {
        let files_arr = repository::get_post_files_by_post_id(pool, parsed_id).await;

        let user_response: UserResponse = convert_user_to_user_res(user.clone(), None);
        let post_response: PostResponse = convert_post_to_post_res(post.clone(), user_response, files_arr);
        return Some((post_response,user.id.to_string()));
    }

    None
}

async fn vec_convert_post_to_post_res(pool:&DbPool,posts: Vec<(Posts, Users)>)->Vec<PostResponse>{
    let mut converted_responses:Vec<PostResponse> = Vec::new();
    for (post, user) in posts {
        let parsed_id = post.id;
        let files_arr = repository::get_post_files_by_post_id(pool, parsed_id).await;
        let user_response: UserResponse = convert_user_to_user_res(user, None);
        let post_response: PostResponse = convert_post_to_post_res(post, user_response, files_arr);
        converted_responses.push(post_response);
    }
    converted_responses
}

pub fn convert_post_to_post_res(post:Posts, user:UserResponse, postfiles:Option<Vec<PostFiles>>)->PostResponse{
    let (url, post_bucket, _) = get_supabase_config();
    let files = postfiles
        .filter(|v| !v.is_empty())
        .map(|v| {
            v.into_iter()
                .map(|pf| format!("{}/storage/v1/object/public/{}/{}",url,post_bucket, pf.files))
                .collect()
        });

    PostResponse {
        id: post.id,
        description: post.description,
        files,
        user,
        views: post.views,
        created_at: post.created_at,
    }
}
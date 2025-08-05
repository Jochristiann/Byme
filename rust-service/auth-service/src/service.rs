use uuid::Uuid;
use model::state::DbPool;
use model::users::{NewToken, NewUsers, RegisterUsers, ResetToken};
use crate::{repository};

pub async fn insert_new_user(pool: &DbPool, new_user:RegisterUsers) -> bool{
    let user = NewUsers{
        id : Uuid::new_v4(),
        name : new_user.name.clone(),
        username: new_user.name,
        email : new_user.email,
        password : new_user.password,
    };
    
    repository::insert_user(pool,&user).await
}

pub async fn change_password(pool: &DbPool, id:String, new_password:String) -> bool{
    let parsed_id = parse_id(id);
    repository::change_password(pool, parsed_id, new_password).await
}

pub async fn insert_token(pool: &DbPool, uid:Uuid, new_password:String) -> (bool, String){
    let new_token = NewToken{
        id: Uuid::new_v4(),
        userid: uid,
        newpassword: new_password
    };

    let result = repository::insert_token(pool, new_token.clone());
    (result.await, new_token.id.to_string())
}

pub async fn get_token(pool:&DbPool, token_id:String) -> Option<ResetToken>{
    let parsed_id = parse_id(token_id);
    let response = repository::get_token(pool,parsed_id).await;
    response
}

pub async fn delete_token(pool:&DbPool, token_id:Uuid) -> bool{
    repository::delete_token(pool, token_id).await
}

fn parse_id (id:String) -> Uuid {
    Uuid::parse_str(&id).unwrap_or_else(|er| {
        eprintln!("{}", er);
        Uuid::nil()
    })
}
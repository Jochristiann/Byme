use uuid::Uuid;
use model::state::DbPool;
use model::users::{NewUsers, RegisterUsers};
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

    let parsed_id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(er) => {eprintln!("{}", er); return false}, // Or handle error properly
    };
    repository::change_password(pool, parsed_id, new_password).await
}
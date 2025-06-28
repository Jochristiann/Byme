use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::Queryable;
use uuid::Uuid;
use crate::model::schema::{users,user_followings};
use crate::model::master::Origins;

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable, Associations)]
#[diesel(belongs_to(Origins, foreign_key = origin_id))]
#[diesel(table_name = users)]
pub struct Users{
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub dob: chrono::NaiveDateTime,
    pub image: String,
    pub role: String,
    pub origin_id: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Identifiable, Associations, Debug, Serialize, Deserialize)]
#[diesel(primary_key(user_id, followed_user_id))]
#[diesel(table_name = user_followings)] 
#[diesel(belongs_to(Users, foreign_key = user_id))]
#[diesel(belongs_to(Users, foreign_key = followed_user_id))]
pub struct UserFollowings {
    pub user_id: Uuid,
    pub followed_user_id: Uuid,
}
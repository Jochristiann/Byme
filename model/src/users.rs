use std::option::Option;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::Queryable;
use uuid::Uuid;
use crate::schema::{users,user_followings};
use crate::master::Origins;

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable, Associations, Insertable)]
#[diesel(belongs_to(Origins, foreign_key = origin_id))]
#[diesel(table_name = users)]
pub struct Users{
    pub id: Uuid,
    pub name: String,   
    pub email: String,
    pub password: String,
    pub dob: Option<NaiveDateTime>,
    pub image: Option<String>,
    pub role: String,
    pub origin_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct RegisterUsers{
    pub name: String,
    pub email: String,
    pub password: String,
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

#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(primary_key(user_id, followed_user_id))]
#[diesel(table_name = user_followings)] 
#[diesel(belongs_to(Users, foreign_key = user_id))]
#[diesel(belongs_to(Users, foreign_key = followed_user_id))]
pub struct UserFollowings {
    pub user_id: Uuid,
    pub followed_user_id: Uuid,
}
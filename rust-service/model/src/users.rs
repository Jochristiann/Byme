use std::option::Option;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{QueryId, Queryable};
use uuid::Uuid;
use crate::schema::{users, userfollowings};
use crate::master::Origins;

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable, Associations, Insertable,QueryId)]
#[diesel(belongs_to(Origins, foreign_key = id))]
#[diesel(table_name = users)]
pub struct Users{
    pub id: Uuid,
    pub name: String,
    pub gender: String,
    pub dob: Option<NaiveDate>,
    pub email: String,
    pub password: String,
    pub image: Option<String>,
    pub role: String,
    pub originid: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Queryable, Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = users)]
pub struct RegisterUsers{
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize,Deserialize, Clone)]
pub struct UserResponse{
    pub id: Uuid,
    pub name: String,
    pub gender:String,
    pub email: String,
    pub dob: Option<NaiveDate>,
    pub image: Option<String>,
    pub role: String,
    pub origin_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Serialize, Deserialize,Clone)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(primary_key(user_id, followed_user_id))]
#[diesel(table_name = userfollowings)]
#[diesel(belongs_to(Users, foreign_key = user_id))]
#[diesel(belongs_to(Users, foreign_key = followed_user_id))]
pub struct UserFollowings {
    pub user_id: Uuid,
    pub followed_user_id: Uuid,
}
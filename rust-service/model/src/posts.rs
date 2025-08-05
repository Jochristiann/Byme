use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::master::Categories;
use crate::schema::{posts};
use crate::users::{UserResponse, Users};

#[derive(Queryable, Identifiable, Associations, Debug, Serialize, Deserialize, Insertable, Clone)]
#[diesel(belongs_to(Users, foreign_key = id))]
#[diesel(table_name = posts)]
pub struct Posts{
    pub id: Uuid,
    pub image: String,
    pub description: String,
    pub views: i64,
    pub categoryid: Uuid,
    pub userid: Uuid,
    pub created_at:NaiveDateTime
}

#[derive(Queryable, Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = posts)]
pub struct NewPost{
    pub id: Uuid,
    pub image: String,
    pub description: String,
    pub categoryid: Uuid,
    pub userid: Uuid
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostRequest{
    pub image: String,
    pub description: String,
    pub category_id: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostResponse{
    pub id: Uuid,
    pub image: String,
    pub description: String,
    pub views: i64,
    pub category: Categories,
    pub user: UserResponse,
    pub created_at:NaiveDateTime
}


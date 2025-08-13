use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::users::{Users};
use crate::schema::{comments};

#[derive(Queryable, Identifiable, Associations, Debug, Serialize, Deserialize, Insertable, Clone)]
#[diesel(belongs_to(Users, foreign_key = id))]
#[diesel(table_name = comments)]
pub struct Comments{
    pub id: Uuid,
    pub message: String,
    pub userid: Uuid,
    pub created_at:NaiveDateTime
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommentRequest{
    pub message: String,
    pub postid: String
}

#[derive(Queryable, Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = comments)]
pub struct NewComment{
    pub id: Uuid,
    pub message: String,
    pub userid: Uuid
}
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::master::Categories;
use crate::comments::Comments;
use crate::schema::{posts, postcomments};
use crate::users::{UserResponse, Users};

#[derive(Queryable, Identifiable, Associations, Debug, Serialize, Deserialize, Insertable, Clone)]
#[diesel(belongs_to(Users, foreign_key = userid))]
#[diesel(belongs_to(Categories, foreign_key = categoryid))]
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
    pub category_id: String
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

#[derive(Insertable,Queryable, Selectable, Associations,Serialize, Deserialize, Clone)]
#[diesel(belongs_to(Comments, foreign_key = commentid))]
#[diesel(belongs_to(Posts, foreign_key = postid))]
#[diesel(table_name = postcomments)]
pub struct PostComments{
    pub postid:Uuid,
    pub commentid: Uuid
}
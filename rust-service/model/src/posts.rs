use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::comments::Comments;
use crate::schema::{posts, postcomments, postfiles};
use crate::users::{UserResponse, Users};

#[derive(Queryable, Identifiable, Associations, Debug, Serialize, Deserialize, Insertable, Clone)]
#[diesel(belongs_to(Users, foreign_key = userid))]
#[diesel(table_name = posts)]
pub struct Posts{
    pub id: Uuid,
    pub description: String,
    pub views: i64,
    pub userid: Uuid,
    pub created_at:NaiveDateTime
}

#[derive(Queryable, Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = posts)]
pub struct NewPost{
    pub id: Uuid,
    pub description: String,
    pub userid: Uuid
}


#[derive(Serialize, Deserialize, Clone)]
pub struct PostRequest{
    pub files: Vec<(String, Vec<u8>)>,
    pub description: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostResponse{
    pub id: Uuid,
    pub files: Option<Vec<String>>,
    pub description: String,
    pub views: i64,
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


#[derive(Queryable, Serialize, Deserialize, Insertable, Clone)]
#[diesel(belongs_to(Posts, foreign_key = postid))]
#[diesel(table_name = postfiles)]
pub struct PostFiles{
    pub postid: Uuid,
    pub files: String
}
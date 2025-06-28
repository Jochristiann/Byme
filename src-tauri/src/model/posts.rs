use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::schema::{posts};
use crate::model::users::Users;

#[derive(Queryable, Identifiable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(Users, foreign_key = user_id))]
#[diesel(table_name = posts)]
pub struct Posts{
    pub id: Uuid,
    pub image: String,
    pub description: String,
    pub views: i32,
    pub category_id: Uuid,
    pub user_id: Uuid,
}
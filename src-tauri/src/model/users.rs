use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::Queryable;
use crate::model::*;

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable, Associations)]
#[diesel(belongs_to(Origin))]
#[table_name = "users"]
pub struct Users{
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub dob: chrono::NaiveDateTime,
    pub image: String,
    pub role: String,
    pub origin_id: Option<uuid::Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
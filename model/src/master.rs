use diesel::prelude::*;
use crate::schema::{origins,categories, violationtypes};

#[derive(Queryable, Insertable)]
#[diesel(table_name = origins)]
pub struct Origins {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = categories)]
pub struct Categories {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = violationtypes)]
pub struct ViolationTypes {
    pub id: uuid::Uuid,
    pub name: String,
}
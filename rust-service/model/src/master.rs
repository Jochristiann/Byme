use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::{origins,categories,violationtypes};

#[derive(Serialize, Deserialize, Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = origins)]
pub struct Origins {
    pub id: Uuid,
    pub country: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = categories)]
pub struct Categories {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Clone, Debug )]
#[diesel(table_name = violationtypes)]
pub struct ViolationTypes {
    pub id: Uuid,
    pub name: String,
}
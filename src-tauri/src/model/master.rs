use diesel::prelude::*;
use diesel::QueryDsl;

#[derive(Queryable, Insertable)]
#[diesel(table_name = origins)]
pub struct Origin {
    pub id: uuid::Uuid,
    pub name: String,
}
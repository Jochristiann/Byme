use uuid::Uuid;

pub struct Users{
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub dob: Option<chrono::NaiveDateTime>,
    pub image: Option<String>,
    pub role: String,
    pub origin_id: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
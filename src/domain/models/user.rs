use serde::{Deserializer, Serializer};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User{
    pub id: i64,
    pub username: String,
    pub email:String,
    pub password: String,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
    pub education: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,

}

#[derive(Debug, Deserialize)]
pub struct UserRegister {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}
use sqlx::SqlitePool;
use crate::domain::models::user::{User, UserRegister};

pub struct UserRepository {
    pool: SqlitePool,

}

impl UserRepository{
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    pub async fn create_user(&self, user:&UserRegister) -> Result<User, sqlx::Error> {
        let hashed_password = bcrypt::hash(user.password.as_bytes(), 8).unwrap();
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password, created_at)
            VALUES (?, ?, ?, datetime('now'))
            RETURNING *
            "#,
            user.username,
            user.email,
            hashed_password
        )
            .fetch_one(&self.pool)
            .await
    }
}

use sqlx::{MySqlPool, mysql::MySqlPoolOptions, Error};
use crate::model::{User, UserInfo};

#[derive(Clone)]
pub struct UserService {
    pub db_pool: MySqlPool,
}

impl UserService {
    pub async fn new() -> Result<Self, Error> {
        let url = std::env::var("DATABASE_URL")
            .expect("❌ DATABASE_URL no está definido (revisá conf/secrets.env o docker-compose)");
        
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await?;

        Ok(Self { db_pool: pool })
    }

    pub async fn list_users(&self) -> Result<Vec<User>, Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, occupation, email, phone FROM users"
        )
        .fetch_all(&self.db_pool)
        .await
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<User, Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, occupation, email, phone FROM users WHERE id = ?"
        )
        .bind(user_id)
        .fetch_one(&self.db_pool)
        .await
    }

    pub async fn create_user(&self, user: UserInfo) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO users (name, occupation, email, phone) VALUES (?, ?, ?, ?)"
        )
        .bind(&user.name)
        .bind(&user.occupation)
        .bind(&user.email)
        .bind(&user.phone)
        .execute(&self.db_pool)
        .await?;

        Ok(result.last_insert_id() as i64)
    }

    pub async fn update_user(&self, id: i32, user: UserInfo) -> Result<(), Error> {
        sqlx::query(
            "UPDATE users SET name = ?, occupation = ?, email = ?, phone = ? WHERE id = ?"
        )
        .bind(&user.name)
        .bind(&user.occupation)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(id)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), Error> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }
}

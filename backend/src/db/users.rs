use anyhow::Result;

use crate::db;

pub struct UserData {
    pub username: String,
    pub password: String,
}

pub async fn get_user_data(user_name: &str) -> Result<UserData> {
    let pool = db::get_pool();
    let user = sqlx::query_as!(
        UserData,
        r#"SELECT username, password
            FROM users WHERE username = $1"#,
        user_name
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn create_user(user_name: &str, password: &str) -> Result<String> {
    let pool = db::get_pool();
    let created_user = sqlx::query_as!(
        UserData,
        r#"INSERT INTO users (username, password)
            VALUES ($1, $2) RETURNING username, password"#,
        user_name,
        password
    )
    .fetch_one(pool)
    .await?;

    Ok(created_user.username)
}

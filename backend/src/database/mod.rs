use std::env;

pub async fn create_sqlite_pool() -> Result<sqlx::SqlitePool, sqlx::Error> {
	dotenvy::from_path("../.env").ok();
	let database_url = env::var("DATABASE_URL").expect("環境変数にDATABASE_URLが設定されていません");
	sqlx::SqlitePool::connect(&database_url).await
}

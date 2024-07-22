use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, thiserror::Error)]
pub enum ArticleRepositoryError {
	#[error("Unexpected Error: [{0}]")]
	Unexpected(String),
	#[error("Not Found, id is {0}")]
	NotFound(String),
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Article {
	pub article_id: String,
	//user_id: String,
	pub body: String,
	pub post_date: String,
}

impl Article {
	fn new(article_id: String, body: String, post_date: String) -> Self {
		Article {
			article_id,
			body,
			post_date,
		}
	}
}

#[derive(Deserialize)]
pub struct CreateArticle {
	// user_id: String,
	pub body: String,
}

#[derive(Deserialize)]
pub struct UpdateArticle {
	pub body: String,
}

#[async_trait]
pub trait ArticleRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
	async fn create(&self, payload: CreateArticle) -> anyhow::Result<Article>;
	async fn get_all(&self) -> anyhow::Result<Vec<Article>>;
	async fn get(&self, article_id: &str) -> Result<Article, ArticleRepositoryError>;
	async fn edit(
		&self,
		payload: UpdateArticle,
		article_id: &str,
	) -> Result<Article, ArticleRepositoryError>;
	async fn delete(&self, article_id: &str) -> Result<(), ArticleRepositoryError>;
}

#[derive(Debug, Clone)]
pub struct ArticleRepositoryForDB {
	pool: SqlitePool,
}

impl ArticleRepositoryForDB {
	pub fn new(pool: SqlitePool) -> Self {
		ArticleRepositoryForDB { pool }
	}
}

#[async_trait]
impl ArticleRepository for ArticleRepositoryForDB {
	async fn create(&self, payload: CreateArticle) -> anyhow::Result<Article> {
		const ID_LEN: u32 = 32;
		let mut include_id = true;
		let mut count: u32 = 0;
		let mut article_id = String::new();
		while include_id {
			article_id = gen_rand_chars(ID_LEN);
			include_id = sqlx::query_as::<_, (bool,)>(r#"SELECT $1 in (SELECT article_id FROM article)"#)
				.bind(&article_id)
				.fetch_one(&self.pool)
				.await?
				.0;
			if count >= 5 {
				return Err(anyhow::Error::msg("登録に失敗しました"));
			}
			count += 1;
		}

		let created_article = sqlx::query_as::<_, Article>(
			r#"INSERT INTO article(article_id, body) VALUES ($1, $2) RETURNING *;"#,
		)
		.bind(&article_id)
		.bind(&payload.body)
		.fetch_one(&self.pool)
		.await?;

		Ok(created_article)
	}
	async fn get_all(&self) -> anyhow::Result<Vec<Article>> {
		let articles = sqlx::query_as::<_, Article>(r#"SELECT * FROM article;"#)
			.fetch_all(&self.pool)
			.await?;
		Ok(articles)
	}
	async fn get(&self, article_id: &str) -> Result<Article, ArticleRepositoryError> {
		let article = sqlx::query_as(r#"SELECT * FROM article WHERE article_id = $1;"#)
			.bind(article_id)
			.fetch_one(&self.pool)
			.await
			.map_err(|e| match e {
				sqlx::Error::RowNotFound => ArticleRepositoryError::NotFound(article_id.to_string()),
				_ => ArticleRepositoryError::Unexpected(e.to_string()),
			})?;
		Ok(article)
	}
	async fn edit(
		&self,
		payload: UpdateArticle,
		article_id: &str,
	) -> Result<Article, ArticleRepositoryError> {
		let updated_article = sqlx::query_as::<_, Article>(
			r#"UPDATE article SET body = $1 WHERE article_id = $2 RETURNING *;"#,
		)
		.bind(payload.body)
		.bind(article_id)
		.fetch_one(&self.pool)
		.await
		.map_err(|e| match e {
			sqlx::Error::RowNotFound => ArticleRepositoryError::NotFound(article_id.to_string()),
			_ => ArticleRepositoryError::Unexpected(e.to_string()),
		})?;
		Ok(updated_article)
	}
	async fn delete(&self, article_id: &str) -> Result<(), ArticleRepositoryError> {
		sqlx::query(r#"DELETE FROM article WHERE article_id = $1"#)
			.bind(article_id)
			.execute(&self.pool)
			.await
			.map_err(|e| match e {
				sqlx::Error::RowNotFound => ArticleRepositoryError::NotFound(article_id.to_string()),
				_ => ArticleRepositoryError::Unexpected(e.to_string()),
			})?;
		Ok(())
	}
}

fn gen_rand_chars(len: u32) -> String {
	use rand::Rng;
	const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	let mut rng = rand::thread_rng();
	(0..len)
		.map(|_| {
			let idx = rng.gen_range(0..CHARSET.len());
			CHARSET[idx] as char
		})
		.collect()
}

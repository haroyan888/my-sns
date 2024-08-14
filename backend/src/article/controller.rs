use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use validator::Validate;

// use crate::modules::gen_string::gen_rand_chars;

#[derive(Debug, thiserror::Error)]
pub enum ArticleRepositoryError {
	#[error("Unexpected Error: [{0}]")]
	Unexpected(String),
	#[error("Not Found, id is {0}")]
	NotFound(String),
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Article {
	#[serde(rename = "id")]
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

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateArticle {
	// user_id: String,
	#[validate(length(min = 1, max = 500))]
	pub body: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateArticle {
	#[validate(length(min = 1, max = 500))]
	pub body: String,
}

#[async_trait]
pub trait ArticleRepository: Clone + Send + Sync + 'static {
	async fn create(&self, payload: CreateArticle) -> anyhow::Result<Article>;
	async fn get_all(&self) -> anyhow::Result<Vec<Article>>;
	async fn find(&self, article_id: &str) -> Result<Article, ArticleRepositoryError>;
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
		let mut tx = self.pool.begin().await?;
		const ID_LEN: u32 = 32;
		let mut is_include_id = true;
		let mut count: u32 = 0;
		let mut article_id = String::new();
		while is_include_id {
			article_id = gen_rand_chars(ID_LEN);
			let is_include_id_res =
				sqlx::query_as::<_, (bool,)>(r#"SELECT $1 in (SELECT article_id FROM article)"#)
					.bind(&article_id)
					.fetch_one(&mut *tx)
					.await;
			if is_include_id_res.is_err() {
				tx.rollback().await?;
				return Err(anyhow::Error::msg("failed to search included article id"));
			}
			is_include_id = is_include_id_res.unwrap().0;
			if count >= 5 {
				tx.rollback().await?;
				return Err(anyhow::Error::msg("登録に失敗しました"));
			}
			count += 1;
		}

		let created_article_res = sqlx::query_as::<_, Article>(
			r#"INSERT INTO article(article_id, body) VALUES ($1, $2) RETURNING *;"#,
		)
		.bind(&article_id)
		.bind(&payload.body)
		.fetch_one(&mut *tx)
		.await;

		match created_article_res {
			Ok(_) => tx.commit().await?,
			Err(_) => tx.rollback().await?,
		}

		Ok(created_article_res?)
	}

	async fn get_all(&self) -> anyhow::Result<Vec<Article>> {
		let mut tx = self.pool.begin().await?;
		let get_all_articles_res =
			sqlx::query_as::<_, Article>(r#"SELECT * FROM article ORDER BY post_date DESC;"#)
				.fetch_all(&mut *tx)
				.await;
		match get_all_articles_res {
			Ok(_) => tx.commit().await?,
			Err(_) => tx.rollback().await?,
		}

		Ok(get_all_articles_res?)
	}

	async fn find(&self, article_id: &str) -> Result<Article, ArticleRepositoryError> {
		let mut tx =
			self.pool.begin().await.map_err(|_| {
				ArticleRepositoryError::Unexpected("failed to start transaction".to_string())
			})?;
		let find_res = sqlx::query_as(r#"SELECT * FROM article WHERE article_id = $1;"#)
			.bind(article_id)
			.fetch_one(&mut *tx)
			.await;

		tx.commit().await.map_err(|_| {
			ArticleRepositoryError::Unexpected("failed to commit transaction".to_string())
		})?;

		let article = find_res.map_err(|e| match e {
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
		let mut tx = self
			.pool
			.begin()
			.await
			.map_err(|e| ArticleRepositoryError::Unexpected(e.to_string()))?;

		let update_res = sqlx::query_as::<_, Article>(
			r#"UPDATE article SET body = $1 WHERE article_id = $2 RETURNING *;"#,
		)
		.bind(payload.body)
		.bind(article_id)
		.fetch_one(&mut *tx)
		.await;

		match update_res {
			Ok(_) => tx.commit().await.map_err(|_| {
				ArticleRepositoryError::Unexpected("failed to commit transaction".to_string())
			})?,
			Err(_) => tx.rollback().await.map_err(|_| {
				ArticleRepositoryError::Unexpected("failed to rollback transaction".to_string())
			})?,
		}

		let updated_article = update_res.map_err(|e| match e {
			sqlx::Error::RowNotFound => ArticleRepositoryError::NotFound(article_id.to_string()),
			_ => ArticleRepositoryError::Unexpected(e.to_string()),
		})?;

		Ok(updated_article)
	}

	async fn delete(&self, article_id: &str) -> Result<(), ArticleRepositoryError> {
		let mut tx = self
			.pool
			.begin()
			.await
			.map_err(|e| ArticleRepositoryError::Unexpected(e.to_string()))?;

		let delete_res = sqlx::query(r#"DELETE FROM article WHERE article_id = $1"#)
			.bind(article_id)
			.execute(&mut *tx)
			.await;

		match delete_res {
			Ok(_) => tx.commit().await.map_err(|_| {
				ArticleRepositoryError::Unexpected("failed to commit transaction".to_string())
			})?,
			Err(_) => tx.rollback().await.map_err(|_| {
				ArticleRepositoryError::Unexpected("failed to rollback transaction".to_string())
			})?,
		}

		delete_res.map_err(|e| match e {
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

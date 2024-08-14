use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use validator::Validate;

use crate::modules::{
	custom_validators::{validate_password, validate_user_id},
	hash::gen_hash_and_salt_from_str,
};

#[derive(Debug, thiserror::Error)]
pub enum AccountRepositoryError {
	#[error("Unexpected Error: [{0}]")]
	Unexpected(String),
	#[error("Not Found, id is {0}")]
	NotFound(String),
	#[error("{0}のアカウントは既に作成されています")]
	Already(String),
}

#[derive(sqlx::FromRow, Serialize, Deserialize, PartialEq, Debug)]
pub struct Account {
	pub user_id: String,
	pub mail_addr: String,
	pub hashed_password: String,
	pub salt: String,
	pub user_name: String,
}

impl Account {
	pub fn new(
		user_id: String,
		mail_addr: String,
		hashed_password: String,
		salt: String,
		user_name: String,
	) -> Self {
		Account {
			user_id,
			mail_addr,
			hashed_password,
			salt,
			user_name,
		}
	}
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct CreateAccount {
	#[validate(length(min = 1, max = 100), custom(function = "validate_user_id"))]
	pub user_id: String,
	#[validate(email)]
	pub mail_addr: String,
	#[validate(length(min = 1, max = 128), custom(function = "validate_password"))]
	pub password: String,
	#[validate(length(min = 1, max = 100))]
	pub user_name: String,
}

#[async_trait]
pub trait AccountRepository {
	async fn create(&self, payload: CreateAccount) -> anyhow::Result<Account>;
	async fn find(&self, user_id: &str) -> Result<Account, AccountRepositoryError>;
	async fn delete(&self, user_id: &str) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct AccountRepositoryForDB {
	pool: SqlitePool,
}

impl AccountRepositoryForDB {
	pub fn new(pool: SqlitePool) -> Self {
		AccountRepositoryForDB { pool }
	}
}

#[async_trait]
impl AccountRepository for AccountRepositoryForDB {
	async fn create(&self, payload: CreateAccount) -> anyhow::Result<Account> {
		let mut tx = self.pool.begin().await?;
		/* パスワードのハッシュ化 */
		let (salt, hashed_password) = gen_hash_and_salt_from_str(&payload.password);
		let res_create_account = sqlx::query_as::<_, Account>(
			r#"INSERT INTO account(user_id, mail_addr, hashed_password, salt, user_name) VALUES ($1, $2, $3, $4, $5) RETURNING *;"#,
		)
			.bind(payload.user_id)
			.bind(payload.mail_addr)
			.bind(hashed_password)
			.bind(salt)
			.bind(payload.user_name)
			.fetch_one(&mut *tx)
			.await;

		match &res_create_account {
			Ok(_) => tx.commit().await?,
			Err(_) => tx.rollback().await?,
		};

		Ok(res_create_account?)
	}

	async fn find(&self, user_id: &str) -> Result<Account, AccountRepositoryError> {
		let mut tx =
			self.pool.begin().await.map_err(|_| {
				AccountRepositoryError::Unexpected("failed to start transaction".to_string())
			})?;
		let account = sqlx::query_as(r#"SELECT * FROM account WHERE user_id=$1;"#)
			.bind(user_id)
			.fetch_one(&mut *tx)
			.await
			.map_err(|e| match e {
				sqlx::Error::RowNotFound => AccountRepositoryError::NotFound(user_id.to_string()),
				_ => AccountRepositoryError::Unexpected(e.to_string()),
			})?;

		tx.commit().await.map_err(|_| {
			AccountRepositoryError::Unexpected("failed to commit transaction".to_string())
		})?;

		Ok(account)
	}

	async fn delete(&self, user_id: &str) -> anyhow::Result<()> {
		let mut tx = self.pool.begin().await?;
		let res_delete_account = sqlx::query(r#"DELETE FROM account WHERE user_id=$1;"#)
			.bind(user_id)
			.execute(&mut *tx)
			.await;

		match &res_delete_account {
			Ok(_) => tx.commit().await?,
			Err(_) => tx.rollback().await?,
		};
		match res_delete_account {
			Ok(_) => return Ok(()),
			Err(e) => return Err(e.into()),
		}
	}
}

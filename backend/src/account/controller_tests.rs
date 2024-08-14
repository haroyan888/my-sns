use super::super::database;
use super::*;
use crate::modules::hash::gen_hash_from_str_and_salt;
use controller::{AccountRepository, AccountRepositoryError};

#[tokio::test]
async fn account_crud_scenario() {
	dotenvy::dotenv().ok();
	/* データベース */
	let pool = database::create_sqlite_pool()
		.await
		.expect("データベースの接続に失敗しました");
	let account_repository = controller::AccountRepositoryForDB::new(pool.clone());

	/* create */
	let create_account = controller::CreateAccount {
		user_id: "test-user_id".to_string(),
		mail_addr: "mail@hoge.com".to_string(),
		password: "test0123".to_string(),
		user_name: "test-user".to_string(),
	};
	let created_account = account_repository
		.create(create_account.clone())
		.await
		.expect("[create] returned Err");
	let account_hashed_password =
		gen_hash_from_str_and_salt(&create_account.password, &created_account.salt);
	let new_account = controller::Account::new(
		create_account.user_id.clone(),
		create_account.mail_addr.clone(),
		account_hashed_password,
		created_account.salt.clone(),
		create_account.user_name.clone(),
	);
	assert_eq!(new_account, created_account);
	println!("[create] success!");

	/* find */
	let found_account = account_repository
		.find(&created_account.user_id)
		.await
		.expect("[find] returned Err");
	assert_eq!(new_account, found_account);
	println!("[find] success!");

	/* delete */
	account_repository
		.delete(&new_account.user_id)
		.await
		.expect("[delete] returned Err");
	let find_deleted_account = account_repository.find(&new_account.user_id).await;
	let Err(find_result) = find_deleted_account else {
		panic!("[delete] found account. failed delete account");
	};
	match find_result {
		AccountRepositoryError::NotFound(_) => println!("[delete] success!"),
		_ => panic!("[delete] returned Err"),
	}

	pool.close().await;
}

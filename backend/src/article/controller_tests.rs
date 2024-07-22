use controller::ArticleRepository;

use super::super::database;
use super::*;

#[tokio::test]
async fn article_crud_scenario() {
	/* データベース */
	let pool = database::create_sqlite_pool()
		.await
		.expect("データベースの接続に失敗しました");
	let article_repository = controller::ArticleRepositoryForDB::new(pool.clone());

	/* create */
	let new_article_body = "new article!";
	let new_article = controller::CreateArticle {
		body: new_article_body.to_string(),
	};
	let created_article = article_repository
		.create(new_article)
		.await
		.expect("[create] returned Err");
	assert_eq!(new_article_body, created_article.body);

	/* find */
	let found_article = article_repository
		.get(&created_article.article_id)
		.await
		.expect("[find] returned Err");
	assert_eq!(new_article_body, found_article.body);

	/* all */
	let got_articles = article_repository
		.get_all()
		.await
		.expect("[all] returned Err");
	let got_article = got_articles.first().unwrap();
	assert_eq!(new_article_body, got_article.body);

	/* update */
	let update_article_body = "update article!";
	let updated_article = article_repository
		.edit(
			controller::UpdateArticle {
				body: update_article_body.to_string(),
			},
			&created_article.article_id,
		)
		.await
		.expect("[update] returning Err");
	assert_eq!(update_article_body, updated_article.body);

	/* delete */
	article_repository
		.delete(&created_article.article_id)
		.await
		.expect("[delete] returning Err");
	let res = article_repository.get(&created_article.article_id).await;
	assert!(res.is_err());
	let article_rows = sqlx::query(r#"SELECT * FROM article WHERE article_id = $1"#)
		.bind(created_article.article_id.clone())
		.fetch_all(&pool)
		.await
		.expect("[delete] returning Err");
	assert!(article_rows.is_empty());
}

use axum::Router;
use backend::article::{
	controller::{ArticleRepository, ArticleRepositoryForDB},
	handler::create_app_article,
};
use backend::database;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
	// .envファイルの適応
	dotenvy::from_path("./.env").ok();
	// ログレベルの設定
	env_logger::init();

	let host = std::env::var("APP_HOST").expect("APP_HOSTが環境変数に設定されていません");
	let port = std::env::var("APP_PORT").expect("APP_HOSTが環境変数に設定されていません");

	let article_repos = ArticleRepositoryForDB::new(
		database::create_sqlite_pool()
			.await
			.expect("データベースの接続に失敗しました"),
	);

	let app = create_app(article_repos);

	let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
		.await
		.expect("リッスンに失敗しました");

	tracing::debug!("listen to http://localhost:{port}");

	axum::serve(listener, app)
		.await
		.expect("アプリケーションの立ち上げに失敗しました");
}

fn create_app<T: ArticleRepository>(article_repos: T) -> Router {
	Router::new()
		.nest("/article", create_app_article(article_repos))
		.layer(
			CorsLayer::new()
				.allow_origin(Any)
				.allow_methods(Any)
				.allow_headers(Any),
		)
}

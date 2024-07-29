use axum::{
	extract::{Extension, Path},
	http::StatusCode,
	response::IntoResponse,
	routing::get,
	Json, Router,
};
use std::sync::Arc;

use super::super::modules::ValidatedJson;
use super::controller::{ArticleRepository, ArticleRepositoryError, CreateArticle, UpdateArticle};

pub fn create_app_article<T: ArticleRepository>(repository: T) -> Router {
	Router::new()
		.route("/", get(get_all_article::<T>).post(create_article::<T>))
		.route(
			"/:article_id",
			get(get_article::<T>)
				.post(edit_article::<T>)
				.delete(delete_article::<T>),
		)
		.layer(Extension(Arc::new(repository)))
}

async fn create_article<T: ArticleRepository>(
	Extension(ref repository): Extension<Arc<T>>,
	ValidatedJson(payload): ValidatedJson<CreateArticle>,
) -> Result<impl IntoResponse, StatusCode> {
	let article = repository
		.create(payload)
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	Ok((StatusCode::CREATED, Json(article)))
}

async fn get_all_article<T: ArticleRepository>(
	Extension(ref repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
	let articles = repository
		.get_all()
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	Ok((StatusCode::OK, Json(articles)))
}

async fn get_article<T: ArticleRepository>(
	Extension(ref repository): Extension<Arc<T>>,
	Path(article_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
	let article = repository.get(&article_id).await.map_err(|e| match e {
		ArticleRepositoryError::NotFound(_) => StatusCode::NOT_FOUND,
		ArticleRepositoryError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
	})?;

	Ok((StatusCode::OK, Json(article)))
}

async fn edit_article<T: ArticleRepository>(
	Extension(ref repository): Extension<Arc<T>>,
	Path(article_id): Path<String>,
	ValidatedJson(payload): ValidatedJson<UpdateArticle>,
) -> Result<impl IntoResponse, StatusCode> {
	let article = repository
		.edit(payload, &article_id)
		.await
		.map_err(|e| match e {
			ArticleRepositoryError::NotFound(_) => StatusCode::NOT_FOUND,
			ArticleRepositoryError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
		})?;

	Ok((StatusCode::OK, Json(article)))
}

async fn delete_article<T: ArticleRepository>(
	Extension(ref repository): Extension<Arc<T>>,
	Path(article_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
	repository.delete(&article_id).await.map_err(|e| match e {
		ArticleRepositoryError::NotFound(_) => StatusCode::NOT_FOUND,
		ArticleRepositoryError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
	})?;

	Ok(())
}

use axum::{
	async_trait,
	extract::{FromRequest, Request},
	http::StatusCode,
	Json,
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationError};

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
	T: DeserializeOwned + Validate,
	B: Send + Sync,
{
	type Rejection = (StatusCode, String);

	async fn from_request(req: Request, state: &B) -> Result<Self, Self::Rejection> {
		let Json(value) = Json::<T>::from_request(req, state)
			.await
			.map_err(|rejection| {
				let message = format!("Json parse error: [{}]", rejection);
				(StatusCode::BAD_REQUEST, message)
			})?;
		value.validate().map_err(|rejection| {
			let message = format!("Json parse error: [{}]", rejection);
			(StatusCode::BAD_REQUEST, message)
		})?;
		Ok(ValidatedJson(value))
	}
}

pub fn validate_password(value: &str) -> Result<(), ValidationError> {
	let lower_case_letters: Vec<char> = ('a'..='z').collect();
	let upper_case_letters: Vec<char> = ('A'..='Z').collect();
	let numbers: Vec<char> = ('0'..='9').collect();
	let symbols: Vec<char> = vec![
		'!', '#', '$', '&', '`', '(', ')', '+', ':', '=', '?', '[', ']', '^', '{', '}', '*', '/', '~',
		'_', ';', '@', '-', '.',
	];
	let available_chars = [lower_case_letters, upper_case_letters, numbers, symbols].concat();
	return validate_from_chars(value, &available_chars);
}

pub fn validate_user_id(value: &str) -> Result<(), ValidationError> {
	let lower_case_letters: Vec<char> = ('a'..='z').collect();
	let upper_case_letters: Vec<char> = ('A'..='Z').collect();
	let numbers: Vec<char> = ('0'..='9').collect();
	let symbols: Vec<char> = vec!['_', '-'];
	let available_chars = [lower_case_letters, upper_case_letters, numbers, symbols].concat();
	return validate_from_chars(value, &available_chars);
}

fn validate_from_chars(value: &str, check_chars: &[char]) -> Result<(), ValidationError> {
	for pass_char in value.chars() {
		if !check_chars.contains(&pass_char) {
			return Err(ValidationError::new("Validation error!"));
		}
	}

	Ok(())
}

use crypto::digest::Digest;
use crypto::sha2::Sha256;

// use super::gen_string::gen_rand_chars;

pub fn gen_hash_and_salt_from_str(before: &str) -> (String, String) {
	let salt = gen_rand_chars(32);
	let mut sha256 = Sha256::new();
	sha256.input_str(&(before.to_string() + &salt));
	(salt.clone(), sha256.result_str())
}

pub fn gen_hash_from_str_and_salt(before: &str, salt: &str) -> String {
	let mut sha256 = Sha256::new();
	sha256.input_str(&(before.to_string() + &salt));
	sha256.result_str()
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

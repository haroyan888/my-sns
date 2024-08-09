pub fn gen_rand_chars(len: u32) -> String {
	use rand::prelude::*;
	const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	let mut rng = StdRng::from_entropy();
	(0..len)
		.map(|_| {
			let idx = rng.gen_range(0..CHARSET.len());
			CHARSET[idx] as char
		})
		.collect()
}

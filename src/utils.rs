use rand::Rng;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const ID_LENGTH: usize = 6;

pub fn generate_short_id() -> String {
    let mut rng = rand::rng();
    (0..ID_LENGTH)
        .map(|_| CHARSET[rng.random_range(0..CHARSET.len())] as char)
        .collect()
}
use rand::Rng;

use crate::domain::minify::EncodeUrl;

#[derive(Debug, Clone)]
pub struct UUIDEncoder {}

const ALPHANUMERIC: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456789";
const ALIAS_SIZE: i32 = 8;

impl UUIDEncoder {
    pub fn new() -> Self {
        UUIDEncoder {}
    }
}

impl EncodeUrl for UUIDEncoder {
    fn encode(&self, _: &str) -> String {
        let mut rng = rand::thread_rng();

        (0..ALIAS_SIZE)
            .map(|_| {
                let selected = rng.gen_range(0..ALPHANUMERIC.len());
                ALPHANUMERIC.chars().nth(selected).unwrap_or_default()
            })
            .collect::<String>()
    }
}

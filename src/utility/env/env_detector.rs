use dotenv::dotenv;
use std::env;

pub struct EnvDetector;

impl EnvDetector {
    pub fn get_var(key: &str) -> Option<String> {
        dotenv().ok();

        match env::var(key) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    pub fn get_host() -> Option<String> {
        Self::get_var("HOST")
    }

    pub fn get_port() -> Option<String> {
        Self::get_var("PORT")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_port() {
        env::set_var("PORT", "1234");

        assert_eq!(EnvDetector::get_port(), Some("1234".to_string()));
    }

    // 다른 키에 대한 테스트도 필요에 따라 추가 가능
}
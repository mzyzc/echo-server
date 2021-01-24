use std::env;

pub fn is_enabled(setting: &str) -> bool {
    if let Ok(b) = env::var(setting) {
        if b == "1" {
            return true;
        }
    }
    false
}
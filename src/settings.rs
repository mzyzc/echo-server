use std::env;

// Check if a setting is on or off
pub fn is_enabled(setting: &str) -> bool {
    if let Ok(b) = env::var(setting) {
        if b == "1" {
            return true;
        }
    }
    false
}
use std::env;

/// Check if a setting is on or off
pub fn is_enabled(setting: &str) -> bool {
    if let Ok(b) = env::var(setting) {
        if b == "1" {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::settings;
    use std::env;

    #[test]
    fn test_is_enabled() {
        env::set_var("ON", "1");
        env::set_var("OFF", "0");

        assert_eq!(settings::is_enabled("ON"), true);
        assert_eq!(settings::is_enabled("OFF"), false);
    }
}
use std::sync::OnceLock;

pub const APP_NAME: &str = "ADHD Coffee Time";
pub const LANGUAGES: [&str; 10] = ["ja", "en", "es", "fr", "de", "pt", "zh", "ko", "ru", "ar"];

pub fn text(language: &str, key: &str) -> &'static str {
    static CATALOG: OnceLock<serde_json::Value> = OnceLock::new();
    let messages = &CATALOG.get_or_init(|| {
        serde_json::from_str(include_str!("../../src/locales.json"))
            .expect("valid translation catalog")
    })["messages"];
    messages[key][language]
        .as_str()
        .or_else(|| messages[key]["ja"].as_str())
        .unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_labels_are_translated_in_every_language() {
        for language in LANGUAGES {
            for key in [
                "settings",
                "pause",
                "startResume",
                "reset",
                "autoDetectMenu",
                "timerVisible",
                "duration",
                "minutesUnit",
                "quit",
                "tagline",
            ] {
                assert!(!text(language, key).is_empty(), "{language}/{key}");
            }
        }
        assert_eq!(text("en", "settings"), "Settings");
        assert_eq!(text("ar", "quit"), "خروج");
    }
}

use egui::ThemePreference;

/// Parses a theme name into an egui preference.
///
/// Accepts `dark`, `light` and `system`, case- and whitespace-insensitively.
/// Anything else returns `None` with a warning, so a typo in a URL degrades to
/// the viewer's default rather than failing to start.
///
/// ```
/// use rewire_viewer::theme::preference;
///
/// assert_eq!(preference("Light"), Some(egui::ThemePreference::Light));
/// assert_eq!(preference("solarized"), None);
/// ```
#[must_use]
pub fn preference(value: &str) -> Option<ThemePreference> {
    match value.trim().to_ascii_lowercase().as_str() {
        "dark" => Some(ThemePreference::Dark),
        "light" => Some(ThemePreference::Light),
        "system" => Some(ThemePreference::System),
        other => {
            re_log::warn!(
                "Ignoring unknown `theme` value {other:?}; expected `dark`, `light` or `system`."
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_documented_values() {
        assert_eq!(preference("dark"), Some(ThemePreference::Dark));
        assert_eq!(preference("light"), Some(ThemePreference::Light));
        assert_eq!(preference("system"), Some(ThemePreference::System));
    }

    #[test]
    fn ignores_case_and_surrounding_whitespace() {
        assert_eq!(preference(" Light "), Some(ThemePreference::Light));
        assert_eq!(preference("DARK"), Some(ThemePreference::Dark));
    }

    #[test]
    fn rejects_unknown_values() {
        assert_eq!(preference("solarized"), None);
        assert_eq!(preference(""), None);
    }
}

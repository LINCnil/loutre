pub fn parse_bool(s: &str) -> bool {
	matches!(s.to_lowercase().as_str(), "true" | "t")
}

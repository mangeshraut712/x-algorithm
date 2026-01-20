//! Utility functions for the candidate pipeline framework

/// Returns a short name for a type by extracting the last component of the path.
/// This is used for logging and metrics.
pub fn short_type_name(full_name: &str) -> &'static str {
    // We need to return a 'static str, so we leak the string
    // In practice, this is called once per type and the memory is negligible
    let short = full_name.rsplit("::").next().unwrap_or(full_name);
    Box::leak(short.to_string().into_boxed_str())
}

/// Placeholder for additional utility functions
pub fn placeholder() {
    // Utility functions will be added here as needed
}

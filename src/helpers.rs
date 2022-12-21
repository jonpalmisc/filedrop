use uuid::Uuid;

/// Prepend a UUID to `name`.
pub fn prepend_uuid(name: &str) -> String {
    format!(
        "{}-{}",
        Uuid::new_v4().to_string().to_uppercase().replace('-', ""),
        name
    )
}

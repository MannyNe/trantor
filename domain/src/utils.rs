pub(crate) fn generate_id() -> String {
    ulid::Ulid::new().to_string()
}

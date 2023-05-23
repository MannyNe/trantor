use ulid::Ulid;

pub fn generate_id() -> String {
    Ulid::new().to_string()
}

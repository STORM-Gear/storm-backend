pub fn get_env_var(name: &'static str) -> String {
    std::env::var(name).expect(format!("'{name}' env var required").as_str())
}

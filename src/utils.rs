pub fn get_env_var(name: &'static str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("'{name}' env var required"))
}

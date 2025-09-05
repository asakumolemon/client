pub struct AppState {
    pub user_id: i32,
    pub username: String,
    pub token: String,
    pub safe_level: i32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            user_id: 0,
            username: String::new(),
            token: String::new(),
            safe_level: 0,
        }
    }
}
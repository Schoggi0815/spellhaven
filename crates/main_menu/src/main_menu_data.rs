pub struct MainMenuData {
    pub seed: String,
    pub server_ip: String,
}

impl Default for MainMenuData {
    fn default() -> Self {
        Self {
            seed: "Seed".into(),
            server_ip: "".into(),
        }
    }
}

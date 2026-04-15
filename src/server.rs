//! Core server logic, PvP mechanic switching, plugin API stub

#[allow(dead_code)]
pub enum PvpMode {
    Legacy18,
    Modern,
}

pub struct Server {
    pub pvp_mode: PvpMode,
    // Plugin system stub
    // pub plugins: Vec<Box<dyn Plugin>>,
    // Player/session management stub
    // pub players: Vec<Player>,
}

#[allow(dead_code)]
impl Server {
    pub fn new(pvp_mode: PvpMode) -> Self {
        Self { pvp_mode }
    }
    /// Start the main server loop (stub)
    pub fn run(&self) {
        println!("Server loop would start here (not yet implemented)");
    }
}
//! Core server logic, PvP mechanic switching, plugin API stub

pub enum PvpMode {
    Legacy18,
    Modern,
}

pub struct Server {
    pub pvp_mode: PvpMode,
    // TODO: Add fields for plugin system, player/session management, etc.
}

impl Server {
    pub fn new(pvp_mode: PvpMode) -> Self {
        Self { pvp_mode }
    }
    // TODO: Add server loop, plugin API, etc.
}
//! Basic chat system for Scissor
//! Handles player messages and server broadcasts.

use crate::core::player_manager::PlayerManager;

pub struct ChatManager;

impl ChatManager {
    pub fn send_message(player: &str, msg: &str) {
        println!("<{}> {}", player, msg);
        // In multiplayer, broadcast to all players
    }

    pub fn broadcast(msg: &str, players: &PlayerManager) {
        for p in players.online() {
            println!("[Broadcast to {}] {}", p.name, msg);
        }
    }
}

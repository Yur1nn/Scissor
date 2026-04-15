#![allow(dead_code)]
//! World control: time, PvP, block permissions, etc.

#[allow(dead_code)]
pub struct WorldControl {
    pub allow_pvp: bool,
    pub allow_place: bool,
    pub allow_break: bool,
    pub allow_break_origin: bool,
    pub time: u32,
}

#[allow(dead_code)]
impl WorldControl {
    pub fn new() -> Self {
        Self {
            allow_pvp: true,
            allow_place: false,
            allow_break: false,
            allow_break_origin: false,
            time: 0,
        }
    }
    /// Set the world time
    pub fn set_time(&mut self, time: u32) {
        self.time = time;
    }

    /// Enable or disable PvP
    pub fn set_pvp(&mut self, allow: bool) {
        self.allow_pvp = allow;
    }

    /// Enable or disable block placing
    pub fn set_place(&mut self, allow: bool) {
        self.allow_place = allow;
    }

    /// Enable or disable block breaking
    pub fn set_break(&mut self, allow: bool) {
        self.allow_break = allow;
    }

    /// Enable or disable breaking origin blocks
    pub fn set_break_origin(&mut self, allow: bool) {
        self.allow_break_origin = allow;
    }
}
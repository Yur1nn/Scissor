//! World control: time, PvP, block permissions, etc.

pub struct WorldControl {
    pub allow_pvp: bool,
    pub allow_place: bool,
    pub allow_break: bool,
    pub allow_break_origin: bool,
    pub time: u32,
}

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
    // TODO: Add methods for world control
}
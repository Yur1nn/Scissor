//! Block interaction hooks for Scissor
//! Events for block place, break, and use.

pub enum BlockEventType {
    Place,
    Break,
    Use,
}

pub struct BlockEvent<'a> {
    pub event_type: BlockEventType,
    pub player: &'a str,
    pub position: (i32, i32, i32),
}

pub trait BlockEventHandler {
    fn handle(&self, event: &BlockEvent);
}

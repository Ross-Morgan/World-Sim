use chrono::Duration;

use crate::event::EventSummary;

pub trait Simulate {
    /// Simulate object
    #[allow(unused_variables)]
    fn advance(&mut self, time_delta: Duration) -> Vec<EventSummary>;
}

use crate::traits::Simulate;
use super::Country;

impl Simulate for Country {
    fn advance(&mut self, time_delta: Duration) -> Vec<EventSummary> {
        self.people
            .iter_mut()
            .map(|p| p.advance(time_delta))
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
    }
}

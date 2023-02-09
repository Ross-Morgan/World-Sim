use std::ops::Range;

use chrono::{Duration, NaiveDateTime};
use rand::{thread_rng, Rng};

use crate::{
    event::{Event, EventLoop, EventSummary},
    id::ID,
    traits::Simulate,
};

pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

#[derive(Clone)]
pub struct Person {
    pub uuid: ID,
    pub born: NaiveDateTime,

    age: Duration,
}

impl Person {
    /// Construct a new person
    pub(crate) fn new(event_loop: &EventLoop) -> Self {
        Self {
            born: event_loop.current_time,
            age: random_age(0..30),
            uuid: ID::next(),
        }
    }

    /// Person's age in years
    pub fn age(&self) -> usize {
        self.age_in(TimeUnit::Years)
    }

    pub fn age_in(&self, unit: TimeUnit) -> usize {
        match unit {
            TimeUnit::Seconds => self.age.num_seconds().abs() as usize,
            TimeUnit::Minutes => self.age.num_minutes().abs() as usize,
            TimeUnit::Hours => self.age.num_hours().abs() as usize,
            TimeUnit::Days => self.age.num_days().abs() as usize,
            TimeUnit::Weeks => self.age.num_weeks().abs() as usize,
            TimeUnit::Months => (self.age_in(TimeUnit::Days) as f64 / 31.0).floor() as usize,
            TimeUnit::Years => (self.age_in(TimeUnit::Days) as f64 / 365.0).floor() as usize,
        }
    }

    /// Person's date of birth
    pub fn date_born(&self) -> NaiveDateTime {
        self.born
    }
}

impl Simulate for Person {
    fn advance(&mut self, time_inc: Duration) -> Vec<EventSummary> {
        let mut v = vec![];

        let prev_age_days = self.age() * 365;

        self.age = self.age + time_inc;

        let new_age_days = time_inc.num_days() as usize;

        if new_age_days - prev_age_days >= 365 {
            v.push(EventSummary(self.uuid, Event::PersonAgeUp(self.age())))
        }

        v
    }
}

fn random_age(age_range: Range<usize>) -> Duration {
    Duration::days(365 * thread_rng().gen_range(age_range) as i64)
}

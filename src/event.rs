use std::collections::HashMap;

use chrono::{Duration, NaiveDateTime};
use rayon::prelude::*;

use crate::{id::ID, objects::Object, traits::Simulate};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    PersonAgeUp(usize),
    PersonDied(ID, NaiveDateTime),
    PersonBorn(ID, NaiveDateTime),
}

#[derive(Copy, Clone, Debug)]
pub struct EventSummary(pub(crate) ID, pub(crate) Event);

impl EventSummary {
    pub fn affected_object_id(&self) -> ID {
        self.0
    }
    pub fn event(&self) -> Event {
        self.1
    }
}

pub struct EventLoop {
    pub start_time: NaiveDateTime,
    pub time_inc: Duration,
    pub current_time: NaiveDateTime,
    pub objects: Vec<Object>,
    pub total_events: usize,
}

impl EventLoop {
    pub fn new(start: NaiveDateTime, increment: Duration) -> Self {
        Self {
            start_time: start,
            time_inc: increment,
            current_time: start,
            total_events: 0,
            objects: vec![],
        }
    }

    pub fn run(&mut self) -> (NaiveDateTime, Vec<EventSummary>) {
        self.start_time += self.time_inc;

        let time_delta = self.time_inc;

        let events = self
            .objects
            .iter_mut()
            .map(|object| object.advance(time_delta))
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        self.total_events += events.len();

        (self.current_time, events)
    }

    pub fn run_parallel(&mut self) -> (NaiveDateTime, Vec<EventSummary>) {
        self.start_time += self.time_inc;

        let time_delta = self.time_inc;

        let events = self
            .objects
            .par_iter_mut()
            .map(|object| object.advance(time_delta))
            .into_par_iter()
            .flatten_iter()
            .collect::<Vec<_>>();

        self.total_events += events.len();

        (self.current_time, events)
    }

    pub fn run_n(&mut self, n: usize) -> HashMap<NaiveDateTime, Vec<EventSummary>> {
        let mut map = HashMap::new();

        for _ in 0..n {
            let (sub_date, sub_events) = self.run();
            map.insert(sub_date, sub_events);
        }

        map
    }

    pub fn run_n_parallel(&mut self, n: usize) -> HashMap<NaiveDateTime, Vec<EventSummary>> {
        let mut map = HashMap::new();

        for _ in 0..n {
            let (sub_date, sub_events) = self.run_parallel();
            map.insert(sub_date, sub_events);
        }

        map
    }

    pub fn add_object(&mut self, o: Object) {
        self.objects.push(o)
    }
}

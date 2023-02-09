use std::sync::{Arc, Mutex};

use chrono::Duration;

use crate::{country::Country, event::EventSummary, person::Person, traits::Simulate};

pub enum Object {
    Country(Arc<Mutex<Country>>),
    Person(Arc<Mutex<Person>>),
}

impl Simulate for Object {
    fn advance(&mut self, time_delta: Duration) -> Vec<EventSummary> {
        match self {
            Object::Country(c) => generic_advance(c, time_delta),
            Object::Person(p) => generic_advance(p, time_delta),
        }
    }
}

fn generic_advance<T: Simulate>(o: &mut Arc<Mutex<T>>, time_delta: Duration) -> Vec<EventSummary> {
    match (**o).try_lock() {
        Ok(mut mg) => mg.advance(time_delta),
        Err(_) => vec![],
    }
}

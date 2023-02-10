mod untracked_new;

use std::sync::{Arc, Mutex};

use chrono::Duration;

use crate::event::{EventLoop, EventSummary};
use crate::id::ID;
use crate::objects::Object;
use crate::person::Person;
use crate::traits::Simulate;

/// Basic representation of a Country
pub struct Country {
    pub name: String,
    pub population: usize,
    pub people: Vec<Person>,
    pub uuid: ID,
}

impl Country {
    /// Construct a country
    pub fn new<T: ToString>(name: T, event_loop: &mut EventLoop) -> Arc<Mutex<Self>> {
        let c = Arc::new(Mutex::new(Self {
            name: name.to_string(),
            population: 0,
            people: vec![],
            uuid: ID::next(),
        }));

        event_loop.add_object(Object::Country(Arc::clone(&c)));

        c
    }

    pub fn new_untracked<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            population: 0,
            people: vec![],
            uuid: ID::next(),
        }
    }

    pub fn new_with_random_population<T: ToString>(
        name: T,
        population: usize,
        event_loop: &mut EventLoop,
    ) -> Arc<Mutex<Self>> {
        let people = (0..population)
            .map(|_| Person::new(event_loop))
            .collect::<Vec<Person>>();

        let c = Arc::new(Mutex::new(Self {
            name: name.to_string(),
            population,
            people,
            uuid: ID::next(),
        }));

        event_loop.add_object(Object::Country(Arc::clone(&c)));

        c
    }
}

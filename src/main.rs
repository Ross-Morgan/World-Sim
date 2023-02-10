use chrono::{Duration, Utc};

use world_sim::country::Country;
use world_sim::event::EventLoop;

fn main() {
    let mut event_loop = EventLoop::new(Utc::now().naive_utc(), Duration::days(1));

    println!(
        "Created populations in {}",
        Duration::span(|| {
            Country::new_with_random_population("United Kingdom", 100_000_000, &mut event_loop);
            // Country::new_with_random_population("United States", 331_400_00, &mut event_loop);
        })
    );

    println!(
        "Ran 5 simulations normally in {}",
        Duration::span(|| {
            // event_loop.run_n(5);
        })
    );
    println!(
        "Ran 5 simulations in parallel in {}",
        Duration::span(|| {
            // event_loop.run_n(5);
        })
    );
}

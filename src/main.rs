mod timer;

use crate::timer::Timer;

/*

ui/
layout/

data/
 mod.rs
 serialization.rs
 session.rs
 statistics.rs
 timer.rs

app.rs
config.rs
errors.rs
events.rs
input.rs
tests/

*/

fn main() {
    let mut timer: Timer = Timer::new();

    timer.start();
    timer.stop();

    println!("Elapsed: {}", timer.elapsed().as_secs_f64());
}

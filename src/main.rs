use std::{thread, time};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use ctrlc;
use rust_gpiozero::LED;

fn main() {
    let keep_going = Arc::new(AtomicBool::new(true));
    let kg = keep_going.clone();

    let red = LED::new(9);
    let amber = LED::new(10);
    let green = LED::new(11);

    ctrlc::set_handler(move || {
       kg.store(false, Ordering::SeqCst);
    }).unwrap();

    red.off();
    amber.off();
    green.off();

    while keep_going.load(Ordering::SeqCst) {
        // Red
        red.on();
        thread::sleep(time::Duration::from_secs(3));

        // Red and Yellow
        amber.on();
        thread::sleep(time::Duration::from_secs(1));

        // Green
        red.off();
        amber.off();
        green.on();
        thread::sleep(time::Duration::from_secs(5));

        // Yellow
        green.off();
        amber.on();
        thread::sleep(time::Duration::from_secs(2));

        // Yellow off
        amber.off();
    }

    red.off();
    amber.off();
    green.off();
}

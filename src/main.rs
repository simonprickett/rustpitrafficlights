use std::{process, thread, time};
use ctrlc;

fn all_lights_off() {
    // TODO turn all the lights off...
    println!("All lights off");
}

fn main() {
    ctrlc::set_handler(move || {
        all_lights_off();
        process::exit(0);
    }).unwrap();

    all_lights_off();

    loop {
        // Red
        println!("Red");
        thread::sleep(time::Duration::from_secs(3));

        // Red and Yellow
        println!("Red and Yellow");
        thread::sleep(time::Duration::from_secs(1));

        // Green
        println!("Green");
        thread::sleep(time::Duration::from_secs(5));

        // Yellow
        println!("Yellow");
        thread::sleep(time::Duration::from_secs(2));

        // Yellow off
        println!("Yellow off");
    }
}

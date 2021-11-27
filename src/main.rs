use std::{process, thread, time};
use ctrlc;
use rust_gpiozero::LED;

fn main() {
    let red = LED::new(9);
    let amber = LED::new(10);
    let green = LED::new(11);

    //fn all_lights_off() {
    //    RED.off();
    //    AMBER.off();
    //    GREEN.off();
    //}

    red.off();
    amber.off();
    green.off();

    ctrlc::set_handler(move || {
        //all_lights_off();
        process::exit(0);
    }).unwrap();

    //all_lights_off();

    loop {
        // Red
        println!("Red");
        red.on();
        thread::sleep(time::Duration::from_secs(3));

        // Red and Yellow
        println!("Red and Yellow");
        amber.on();
        thread::sleep(time::Duration::from_secs(1));

        // Green
        println!("Green");
        red.off();
        amber.off();
        green.on();
        thread::sleep(time::Duration::from_secs(5));

        // Yellow
        println!("Yellow");
        green.off();
        amber.on();
        thread::sleep(time::Duration::from_secs(2));

        // Yellow off
        println!("Yellow off");
        amber.off();
    }
}

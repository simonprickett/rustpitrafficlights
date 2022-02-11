use crossbeam_channel::{bounded, select, tick, Receiver};
use ctrlc;
use rust_gpiozero::LED;
use std::collections::HashMap;
use std::time::Duration;
use log;
use std::fmt;

fn ctrlc_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;
    Ok(receiver)
}

// The various states you can be in
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum TrafficState {
    Red,
    RedAmber,
    Green,
    Amber,
}

struct StateInfo {
    which_leds: (bool, bool, bool),
    how_long: Duration,
    next: TrafficState,
}

// The three LEDs
struct Leds {
    red: LED,
    amber: LED,
    green: LED,
}

impl fmt::Debug for Leds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "red: {}, amber: {}, green: {}", self.red.value(), self.amber.value(), self.green.value())
    }
}

// We turn them all off at startup and shutdown, so add a convenience method
fn reset(leds: &Leds) {
    log::debug!("All LEDs off");
    leds.red.off();
    leds.amber.off();
    leds.green.off();
}

fn main() {
    env_logger::init();
    let ctrl_c_events = ctrlc_channel().unwrap();

    // create the leds object to reference the correct pins for the LEDs
    let mut leds = Leds {
        red: LED::new(9),
        amber: LED::new(10),
        green: LED::new(11),
    };

    // here we define all the states the LED can be in, and how things behave
    let state_info = HashMap::from([
        (TrafficState::Red, StateInfo{
            which_leds: (true, false, false),
            how_long: Duration::from_secs(3),
            next: TrafficState::RedAmber,
        }),
        (TrafficState::RedAmber, StateInfo{
            which_leds: (true, true, false),
            how_long: Duration::from_secs(1),
            next: TrafficState::Green,
        }),
        (TrafficState::Green, StateInfo{
            which_leds: (false, false, true),
            how_long: Duration::from_secs(1),
            next: TrafficState::Amber,
        }),
        (TrafficState::Amber, StateInfo{
            which_leds: (false, true, false),
            how_long: Duration::from_secs(1),
            next: TrafficState::Red,
        }),
    ]);

    let mut state = TrafficState::Red;

    loop {
        let want = state_info.get(&state).unwrap();
        // set the LEDs as desired
        let (red_want, amber_want, green_want) = want.which_leds;
        if red_want ^ leds.red.value() {
            leds.red.toggle();
        }
        if amber_want ^ leds.amber.value() {
            leds.amber.toggle();
        }
        if green_want ^ leds.green.value() {
            leds.green.toggle();
        }

        log::debug!("{:?}: {:?}", state, leds);

        // compute the delay for the current state
        let ticks = tick(want.how_long);
        select! {
            recv(ticks) -> _ => state = want.next,
            recv(ctrl_c_events) -> _ => {
                reset(&leds);
                return;
            }
        }
    }
}

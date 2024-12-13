#![no_std]
#![no_main]

use arduino_hal::{hal::port::{PB5, PD2, PD7}, port::{mode::{Floating, Input, Output}, Pin}};
use panic_halt as _;

const THRESHOLD_MILLIS: u16 = 3*1000;
const DELAY_MILLIS: u16 = 100;
const START_TIME_MILLIS: u16 = 0;


trait LoopSound {
    fn start_sound(&mut self);
    fn stop_sound(&mut self);
}

impl LoopSound for Pin<Output, PD7> {

    fn start_sound(&mut self) {
        self.set_high();
    }

    fn stop_sound(&mut self) {
        self.set_low();
    }
}

trait Lighter {
    fn start_light(&mut self);
    fn stop_light(&mut self);
}

impl Lighter for Pin<Output, PB5> {

    fn start_light(&mut self) {
        self.set_high();
    }

    fn stop_light(&mut self) {
        self.set_low();
    }
}

trait OpenDoorDetector {
    fn is_open(&self) -> bool;
}

impl OpenDoorDetector for Pin<Input<Floating>,PD2> {
    fn is_open(&self) -> bool {
        return self.is_high();
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

     let mut led = pins.d13.into_output();
     let mut buzzer = pins.d7.into_output();
     let infrared = pins.d2.into_floating_input();

     let mut ms_since_door_open = START_TIME_MILLIS;

     led.stop_light();
     buzzer.stop_sound();
     
     loop {
        arduino_hal::delay_ms(DELAY_MILLIS);

        if infrared.is_open() {
            ms_since_door_open += DELAY_MILLIS
        } else {
            ms_since_door_open = START_TIME_MILLIS
        }

        if ms_since_door_open > THRESHOLD_MILLIS {
            buzzer.start_sound();
            led.start_light();
        } else {
            buzzer.stop_sound();
            led.stop_light();
        }
    }
}

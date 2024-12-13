#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use panic_halt as _;

const THRESHOLD_MILLIS: u16 = 3*1000;
const DELAY_MILLIS: u16 = 100;
const START_TIME_MILLIS: u16 = 0;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

     let mut led = pins.d13.into_output();
     let mut buzzer = pins.d7.into_output();
     let infrared = pins.d2.into_floating_input();
     let mut ms_since_door_open = START_TIME_MILLIS;

     led.set_low();
     buzzer.set_low();

     loop {
        arduino_hal::delay_ms(DELAY_MILLIS);
        // Door opened
        if infrared.is_high() {
            ms_since_door_open += DELAY_MILLIS
        } else {
            ms_since_door_open = START_TIME_MILLIS
        }
        if ms_since_door_open > THRESHOLD_MILLIS {
            led.set_high();
            buzzer.set_high();
        } else {
            led.set_low();
            buzzer.set_low();
        }
    }
}

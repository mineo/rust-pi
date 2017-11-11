extern crate rppal;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;
use std::thread::sleep;
use std::time::Duration;

// The pins that are connected to LEDs. These are in the order in which they
// should light up in RaspberryGpio.cycle().
static PINS: [u8; 6] = [13, 19, 26, 21, 20, 16];
static HOWLONG: u64 = 100;

struct RaspberryGpio {
    gpio: Gpio,
    sleep_duration: Duration,
    pins: Vec<u8>,
}

impl RaspberryGpio {
    pub fn new(mut gpio: Gpio, sleep_duration: Duration, pins: Vec<u8>) -> RaspberryGpio {
        for &pin in &pins {
            gpio.set_mode(pin, Mode::Output);
        }

        RaspberryGpio {
            gpio: gpio,
            sleep_duration: sleep_duration,
            pins: pins,
        }
    }

    fn wait(&self) {
        sleep(self.sleep_duration);
    }

    fn off(&self, pin: u8) {
        self.gpio.write(pin, Level::Low);
    }

    fn off_slice(&self, pins: &[u8]) {
        for &pin in pins {
            self.off(pin);
        }
    }

    fn on(&self, pin: u8) {
        self.gpio.write(pin, Level::High);
    }

    fn on_slice(&self, pins: &[u8]) {
        for &pin in pins {
            self.on(pin);
        }
    }

    pub fn cycle(&self) {
        for &pin in &self.pins {
            self.on(pin);
            self.wait();
            self.off(pin);
        }
    }

    pub fn flash(&self) {
        for _ in 1..3 {
            self.on_slice(self.pins.as_slice());
            self.wait();
            self.off_slice(self.pins.as_slice());
            self.wait();
        }
    }

    pub fn trail(&self) {
        let mut lastpin: u8 = 0;
        for &pin in &self.pins {
            self.on(pin);
            self.wait();
            if lastpin > 0 {
                self.off(pin);
            }
            lastpin = pin;
        }
    }

    pub fn cross(&self) {
        let cap = self.pins.len() / 2;

        // All elements at even indexes
        let mut subset1: Vec<u8> = Vec::with_capacity(cap + 1);
        // All elements at uneven indexes
        let mut subset2: Vec<u8> = Vec::with_capacity(cap);

        for (i, &pin) in self.pins.iter().enumerate() {
            if (i % 2) == 0 {
                subset1.push(pin);
            } else {
                subset2.push(pin);
            }
        }

        for _ in 1..7 {
            self.on_slice(subset1.as_slice());
            self.wait();
            self.off_slice(subset1.as_slice());
            self.wait();
            self.on_slice(subset2.as_slice());
            self.wait();
            self.off_slice(subset2.as_slice());
            self.wait();
        }
    }
}

pub fn demo(gpio: Gpio, pins: Vec<u8>) {
    let rgpio = RaspberryGpio::new(gpio, Duration::from_millis(HOWLONG), pins);
    rgpio.cycle();
    rgpio.flash();
    rgpio.trail();
    rgpio.cross();
}

fn main() {
    let info = DeviceInfo::new().expect("Couldn't get device info");
    println!("Model: {} (SoC: {})", info.model(), info.soc());

    let gpio = Gpio::new().expect("Couldn't get gpio");
    let mut pins = PINS.to_vec();

    for _ in 1..5 {
        pins.extend(PINS.iter().cloned());
    }

    sleep(Duration::from_secs(5));

    demo(gpio, pins);
}

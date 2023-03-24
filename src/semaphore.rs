use core::convert::Infallible;

use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::gpio::{
    bank0::{Gpio11, Gpio14, Gpio13},
    Output, Pin, PushPull,
};

pub struct Semaphore {
    green: Pin<Gpio14, Output<PushPull>>,
    yellow: Pin<Gpio13, Output<PushPull>>,
    red: Pin<Gpio11, Output<PushPull>>,
}

impl Semaphore {
    pub fn new(
        green: Pin<Gpio14, Output<PushPull>>,
        yellow: Pin<Gpio13, Output<PushPull>>,
        red: Pin<Gpio11, Output<PushPull>>,
    ) -> Self {
        Self { green, yellow, red }
    }
}

impl Semaphore {
    pub fn green_light(&mut self) -> Result<(), Infallible> {
        self.green.set_high().unwrap();
        self.red.set_low().unwrap();
        self.yellow.set_low().unwrap();
        Ok(())
    }
    pub fn yellow_light(&mut self) -> Result<(), Infallible> {
        self.green.set_low().unwrap();
        self.red.set_low().unwrap();
        self.yellow.set_high().unwrap();
        Ok(())
    }
    pub fn red_light(&mut self) -> Result<(), Infallible> {
        self.green.set_low().unwrap();
        self.red.set_high().unwrap();
        self.yellow.set_low().unwrap();
        Ok(())
    }
}

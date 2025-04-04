use core::convert::Infallible;

use embedded_hal::digital::OutputPin;
use rp_pico::hal::gpio::{
    bank0::{Gpio11, Gpio13, Gpio14}, FunctionSio, Pin, PullDown, SioOutput
};

pub struct Semaphore {
    green: Pin<Gpio14, FunctionSio<SioOutput>, PullDown>,
    yellow: Pin<Gpio13, FunctionSio<SioOutput>, PullDown>,
    red: Pin<Gpio11, FunctionSio<SioOutput>, PullDown>,
}

impl Semaphore {
    pub fn new(
        green: Pin<Gpio14, FunctionSio<SioOutput>, PullDown>,
        yellow: Pin<Gpio13, FunctionSio<SioOutput>, PullDown>,
        red: Pin<Gpio11, FunctionSio<SioOutput>, PullDown>,
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

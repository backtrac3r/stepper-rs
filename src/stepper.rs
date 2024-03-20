use crate::joy::MID_VAL;
use embassy_stm32::gpio::Output;
use embassy_time::{Duration, Timer};

pub const MIN_DELAY_MICROS: u64 = 2000;
pub const MAX_DELAY_MICROS: u64 = 15000;

pub struct Stepper {
    dir_pin: Output<'static>,
    step_pin: Output<'static>,
    delay: Duration,
}

impl Stepper {
    #[must_use]
    pub fn new(dir_pin: Output<'static>, step_pin: Output<'static>) -> Self {
        Self {
            dir_pin,
            step_pin,
            delay: Duration::from_micros(MAX_DELAY_MICROS),
        }
    }

    #[must_use]
    pub fn get_delay(&self) -> Duration {
        self.delay
    }

    async fn next_step_right(&mut self) {
        if self.dir_pin.is_set_high() {
            self.dir_pin.set_low();
        }

        self.step_pin.set_high();
        Timer::after(self.delay).await;
        self.step_pin.set_low();
    }

    pub fn update_delay(&mut self, new_delay: u64) {
        self.delay = Duration::from_micros(new_delay);
    }

    async fn next_step_left(&mut self) {
        if self.dir_pin.is_set_low() {
            self.dir_pin.set_high();
        }

        self.step_pin.set_high();
        Timer::after(self.delay).await;
        self.step_pin.set_low();
    }

    #[allow(clippy::missing_panics_doc)]
    pub async fn next_step(&mut self, val: u16) {
        if val > MID_VAL {
            self.next_step_right().await;
        } else {
            self.next_step_left().await;
        }
    }
}

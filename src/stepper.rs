use crate::{
    helpers::{MID, MID_ERROR},
    joy::Joy,
};
use embassy_stm32::gpio::Output;
use embassy_time::{Duration, Timer};

pub struct Motor {
    dir_pin: Output<'static>,
    step_pin: Output<'static>,
    delay: Duration,
}

impl Motor {
    #[must_use]
    pub fn new(dir_pin: Output<'static>, step_pin: Output<'static>, delay: Duration) -> Self {
        Self {
            dir_pin,
            step_pin,
            delay,
        }
    }

    async fn next_step_right(&mut self) {
        if self.dir_pin.is_set_high() {
            self.dir_pin.set_low();
        }

        self.step_pin.set_high();
        Timer::after(self.delay).await;
        self.step_pin.set_low();
    }

    async fn next_step_left(&mut self) {
        if self.dir_pin.is_set_low() {
            self.dir_pin.set_high();
        }

        self.step_pin.set_high();
        Timer::after(self.delay).await;
        self.step_pin.set_low();
    }

    pub async fn next_step(&mut self, joy: &mut Joy) {
        let y = joy.get_y();

        let abs_diff = y.abs_diff(MID);

        if abs_diff < MID_ERROR {
            Timer::after(self.delay).await;
            return;
        }

        if y > MID {
            self.next_step_right().await;
        } else {
            self.next_step_left().await;
        }
    }
}

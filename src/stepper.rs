use crate::{
    helpers::{joy_map, mirror_in_range},
    joy::{Joy, JOY_ERROR, MID_VAL},
};
use defmt::println;
use embassy_stm32::gpio::Output;
use embassy_time::{Duration, Timer};

const MIN_DELAY_MICROS: u64 = 2000;
const MAX_DELAY_MICROS: u64 = 10000;

pub struct Motor {
    dir_pin: Output<'static>,
    step_pin: Output<'static>,
    delay: Duration,
}

impl Motor {
    #[must_use]
    pub fn new(dir_pin: Output<'static>, step_pin: Output<'static>, delay_micros: u64) -> Self {
        Self {
            dir_pin,
            step_pin,
            delay: Duration::from_micros(delay_micros),
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

    fn update_delay(&mut self, new_delay: u64) {
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
    pub async fn next_step(&mut self, joy: &mut Joy) {
        let y = joy.get_y();

        let abs_diff = y.abs_diff(MID_VAL);

        if abs_diff < JOY_ERROR {
            Timer::after(self.delay).await;
            return;
        }

        let new_delay = joy_map(
            abs_diff,
            JOY_ERROR,
            MID_VAL + 1,
            MIN_DELAY_MICROS.try_into().unwrap(),
            MAX_DELAY_MICROS.try_into().unwrap(),
        );
        println!("new_delay: {}", new_delay);
        let new_delay: u64 = new_delay.into();
        let new_delay = mirror_in_range(new_delay, MIN_DELAY_MICROS, MAX_DELAY_MICROS);
        println!("y: {}", y);
        println!("abs_diff: {}", abs_diff);
        println!("new_delay: {}", new_delay);
        println!("");

        self.update_delay(new_delay);

        if y > MID_VAL {
            self.next_step_right().await;
        } else {
            self.next_step_left().await;
        }
    }
}

use crate::{
    filter::Filter,
    helpers::{joy_map, mirror_in_range},
    joy::{Joy, JOY_ERROR, MID_VAL},
    stepper::{Stepper, MAX_DELAY_MICROS, MIN_DELAY_MICROS},
};
use defmt::println;
use embassy_time::Timer;

pub struct Launcher {
    joy: Joy,
    joy_filter: Filter,
    y_motor: Stepper,
}

impl Launcher {
    #[must_use]
    pub fn new(joy: Joy, joy_filter: Filter, y_motor: Stepper) -> Self {
        Self {
            joy,
            joy_filter,
            y_motor,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub async fn do_move(&mut self) {
        let y = self.joy.get_y();
        let y = self.joy_filter.filter_value(y);

        let abs_diff = y.abs_diff(MID_VAL);

        if abs_diff < JOY_ERROR {
            Timer::after(self.y_motor.get_delay()).await;
            return;
        }

        let new_delay = joy_map(
            abs_diff,
            JOY_ERROR,
            MID_VAL + 1,
            MIN_DELAY_MICROS.try_into().unwrap(),
            MAX_DELAY_MICROS.try_into().unwrap(),
        );
        println!("y: {}", y);
        println!("new_delay: {}", new_delay);
        let new_delay: u64 = new_delay.into();
        let new_delay = mirror_in_range(new_delay, MIN_DELAY_MICROS, MAX_DELAY_MICROS);
        println!("abs_diff: {}", abs_diff);
        println!("new_delay: {}", new_delay);
        println!("");

        self.y_motor.update_delay(new_delay);

        self.y_motor.next_step(y).await;
    }
}

#[embassy_executor::task]
pub async fn driver(mut l: Launcher) {
    loop {
        l.do_move().await;
    }
}

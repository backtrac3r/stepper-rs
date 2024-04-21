use crate::{
    filter::Filter,
    helpers::{joy_map, mirror_in_range},
    joy::{Joy, JOY_ERROR, MID_VAL},
    stepper::{Stepper, MAX_DELAY_MICROS, MIN_DELAY_MICROS},
};
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
        // берем показания джостика для мотора, который отвечает за ось Y
        let y = self.joy.get_y();
        let y = self.joy_filter.filter_value(y);

        // если джостик в дефолтном положении, спим и пропускаем итерацию,
        // иначе считаем задержку для нового шага мотора
        let abs_diff = y.abs_diff(MID_VAL);
        if abs_diff < JOY_ERROR {
            Timer::after(self.y_motor.get_delay()).await;
            return;
        }

        // считаем задержку для нового шага мотора
        let new_delay = Self::calc_motor_delay(abs_diff);

        // обновляем задержку шага
        self.y_motor.update_delay(new_delay);

        // делаем шаг
        self.y_motor.next_step(y).await;
    }

    fn calc_motor_delay(abs_diff: u16) -> u64 {
        let new_delay = joy_map(
            abs_diff,
            JOY_ERROR,
            MID_VAL + 1,
            MIN_DELAY_MICROS.try_into().unwrap(),
            MAX_DELAY_MICROS.try_into().unwrap(),
        );

        let new_delay: u64 = new_delay.into();

        mirror_in_range(new_delay, MIN_DELAY_MICROS, MAX_DELAY_MICROS)
    }
}

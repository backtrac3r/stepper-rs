use crate::helpers::center_x;
use embassy_stm32::{
    adc::Adc,
    gpio::Input,
    peripherals::{ADC1, PA0, PA1},
};

pub const X_MIN: u16 = 0;
pub const X_MID: u16 = 3028;
pub const X_MAX: u16 = 4095;
pub const Y_MIN: u16 = 0;
pub const Y_MID: u16 = 3028;
pub const Y_MAX: u16 = 4095;
pub const JOY_ERROR: u16 = 150;
pub const MID_VAL: u16 = 2047;

pub struct Joy {
    adc: Adc<'static, ADC1>,
    button: Input<'static>,
    x_pin: PA0,
    y_pin: PA1,
}

impl Joy {
    #[must_use]
    pub fn new(adc: Adc<'static, ADC1>, button: Input<'static>, x_pin: PA0, y_pin: PA1) -> Self {
        Self {
            adc,
            button,
            x_pin,
            y_pin,
        }
    }

    pub fn get_x(&mut self) -> u16 {
        let val = self.adc.read(&mut self.x_pin);
        center_x(val)
    }

    pub fn get_y(&mut self) -> u16 {
        let val = self.adc.read(&mut self.y_pin);
        center_x(val)
    }

    pub fn get_button_state(&mut self) -> bool {
        self.button.is_low()
    }
}

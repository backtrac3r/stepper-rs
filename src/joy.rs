use embassy_stm32::{
    adc::Adc,
    gpio::Input,
    peripherals::{ADC1, PA0, PA1},
};

use crate::helpers::normalize_x;

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
        normalize_x(val)
    }

    pub fn get_y(&mut self) -> u16 {
        let val = self.adc.read(&mut self.y_pin);
        normalize_x(val)
    }

    pub fn get_button_state(&mut self) -> bool {
        self.button.is_low()
    }
}

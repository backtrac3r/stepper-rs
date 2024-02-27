#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate stepper_rs;

use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_stm32::{
    adc::{Adc, Resolution},
    gpio::{Input, Level, Output, Pull, Speed},
    Config,
};
use embassy_time::Delay;
use stepper::Stepper;
use stepper_rs::{
    filter::Filter,
    joy::Joy,
    launcher::{self, Launcher},
    stepper,
};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());

    let dir_pin = Output::new(p.PA9, Level::Low, Speed::VeryHigh);
    let step_pin = Output::new(p.PA8, Level::Low, Speed::VeryHigh);
    let y_motor = Stepper::new(dir_pin, step_pin);

    let mut adc = Adc::new(p.ADC1, &mut Delay);
    adc.set_resolution(Resolution::BITS12);
    let joy_button = Input::new(p.PA10, Pull::Up);
    let joy_filter = Filter::default();
    let joy = Joy::new(adc, joy_button, p.PA0, p.PA1);

    let launcher = Launcher::new(joy, joy_filter, y_motor);

    unwrap!(spawner.spawn(launcher::driver(launcher)));
}

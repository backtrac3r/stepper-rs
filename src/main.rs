#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate stepper_rs;

use defmt::{println, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::{
    adc::{Adc, Resolution},
    gpio::{Input, Level, Output, Pull, Speed},
    Config,
};
use embassy_time::{Delay, Duration};
use stepper::Motor;
use stepper_rs::{joy::Joy, stepper};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());

    let dir_pin = Output::new(p.PA9, Level::Low, Speed::VeryHigh);
    let step_pin = Output::new(p.PA8, Level::Low, Speed::VeryHigh);
    let duration = Duration::from_micros(2000);
    let motor = Motor::new(dir_pin, step_pin, duration);

    let mut adc = Adc::new(p.ADC1, &mut Delay);
    adc.set_resolution(Resolution::BITS12);
    let joy_button = Input::new(p.PA10, Pull::Up);
    let joy = Joy::new(adc, joy_button, p.PA0, p.PA1);

    unwrap!(spawner.spawn(motor_driver(motor, joy)));
}

#[embassy_executor::task]
async fn motor_driver(mut motor: Motor, mut joy: Joy) {
    loop {
        motor.next_step(&mut joy).await;
    }
}

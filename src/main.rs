#![no_std]
#![no_main]

use arduino_hal::simple_pwm::IntoPwmPin;
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::simple_pwm::{Timer3Pwm, Timer4Pwm};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer0 = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);
    let timer1 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);

    let mut d13 = pins.d13.into_output().downgrade();

    let mut d6 = pins.d6.into_output().into_pwm(&timer0);
    let mut d5 = pins.d5.into_output().into_pwm(&timer1);
    let mut d3 = pins.d3.into_output().into_pwm(&timer1);

    let max_duty_d6 = d6.get_max_duty();
    let max_duty_d5 = d6.get_max_duty();
    let max_duty_d3 = d6.get_max_duty();

    let delay_time = 100;

    d6.enable();
    d5.enable();
    d3.enable();

    loop {
        for i in (0..=max_duty_d6).chain((0..=max_duty_d6 - 1).rev()) {
            d6.set_duty(i);
            d13.toggle();
            arduino_hal::delay_ms(delay_time);
        }

        for i in (0..=max_duty_d5).chain((0..=max_duty_d5 - 1).rev()) {
            d5.set_duty(i);
            d13.toggle();
            arduino_hal::delay_ms(delay_time);
        }

        for i in (0..=max_duty_d3).chain((0..=max_duty_d3 - 1).rev()) {
            d3.set_duty(i);
            d13.toggle();
            arduino_hal::delay_ms(delay_time);
        }
    }
}

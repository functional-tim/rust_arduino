#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(dead_code)]

use arduino_hal::simple_pwm::IntoPwmPin;
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::simple_pwm::{Timer3Pwm, Timer4Pwm};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    let pins = arduino_hal::pins!(dp);
    
    let a0 = pins.a0.into_output();
    let d9 = pins.d9.into_pull_up_input();
    let mut d5 = pins.d5.into_output().downgrade();

    loop {
        if d9.is_low() {
            let temp = 5;
            for _ in 0..temp {
                d5.set_high();
                arduino_hal::delay_ms(500);
                d5.set_low();
                arduino_hal::delay_ms(500);
            }
        }
        d5.set_low();
    }


    // Lesson 6 - Active Buzzer
//    let mut d12 = pins.d12.into_output().downgrade();
//    let mut duration: u32 = 500;
//
//    loop {
//        for i in 0..20 {
//            if i < 5 {
//                duration = 500;
//            } else if i < 10 {
//                duration = 300;
//            } else if i < 20 {
//                duration = 100;
//            }
//
//            d12.set_high();
//            arduino_hal::delay_ms(duration);
//            d12.set_low();
//            arduino_hal::delay_ms(duration);
//        }
//        d12.set_high();
//        arduino_hal::delay_ms(5000);
//        d12.set_low();
//    }

    // Lesson 5 - Digital Inputs
//    let d9 = pins.d9.into_pull_up_input();
//    let d8 = pins.d8.into_pull_up_input();
//    let mut d5 = pins.d5.into_output().downgrade();
//
//    loop {
//        if d9.is_low() {
//            d5.set_high();
//        }
//
//        if d8.is_low() {
//            d5.set_low();
//        }
//    }

    // Lesson 4 - RGB LED
//    let pins = arduino_hal::pins!(dp);
//
//    let timer0 = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);
//    let timer1 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);
//
//    let mut d13 = pins.d13.into_output().downgrade();
//
//    let mut d6 = pins.d6.into_output().into_pwm(&timer0);
//    let mut d5 = pins.d5.into_output().into_pwm(&timer1);
//    let mut d3 = pins.d3.into_output().into_pwm(&timer1);
//
//    let max_duty_d6 = d6.get_max_duty();
//    let max_duty_d5 = d6.get_max_duty();
//    let max_duty_d3 = d6.get_max_duty();
//
//    let delay_time = 100;
//
//    d6.enable();
//    d5.enable();
//    d3.enable();
//
//    loop {
//        for i in (0..=max_duty_d6).chain((0..=max_duty_d6 - 1).rev()) {
//            d6.set_duty(i);
//            d13.toggle();
//            arduino_hal::delay_ms(delay_time);
//        }
//
//        for i in (0..=max_duty_d5).chain((0..=max_duty_d5 - 1).rev()) {
//            d5.set_duty(i);
//            d13.toggle();
//            arduino_hal::delay_ms(delay_time);
//        }
//
//        for i in (0..=max_duty_d3).chain((0..=max_duty_d3 - 1).rev()) {
//            d3.set_duty(i);
//            d13.toggle();
//            arduino_hal::delay_ms(delay_time);
//        }
//    }
}

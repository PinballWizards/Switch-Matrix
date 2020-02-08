#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate feather_m0 as hal;
#[cfg(not(feature = "use_semihosting"))]  
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]       
extern crate panic_semihosting;           
extern crate embedded_hal;                

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
//use hal::adc::Adc;                          
//use cortex_m_semihosting::hprintln;       

use hal::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut column_one = pins.a1.into_push_pull_output(&mut pins.port);
    let mut column_two = pins.a2.into_push_pull_output(&mut pins.port);
    let mut column_three = pins.a3.into_push_pull_output(&mut pins.port);
    let mut column_four = pins.a4.into_push_pull_output(&mut pins.port);
    let mut column_five = pins.d10.into_push_pull_output(&mut pins.port);
    let mut led_one = pins.d13.into_push_pull_output(&mut pins.port);
    let mut led_two = pins.a5.into_push_pull_output(&mut pins.port);
    let mut led_three = pins.d5.into_push_pull_output(&mut pins.port);
    let mut led_four = pins.d6.into_push_pull_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let input_one = pins.d11.into_pull_down_input(&mut pins.port);
    let input_two = pins.d12.into_pull_down_input(&mut pins.port);

    loop {
        column_one.set_high().unwrap();                        // set column 1 to high
        if input_one.is_high().unwrap() {                      // input 1 high button 1
             led_one.set_high().unwrap();
             led_two.set_low().unwrap();
             led_three.set_low().unwrap();
             led_four.set_low().unwrap();
        } else if input_two.is_high().unwrap() {               // input 2 high button 6
             led_three.set_high().unwrap();
             led_two.set_high().unwrap();
             led_one.set_low().unwrap();
             led_four.set_low().unwrap();
        } else {
             led_one.set_low().unwrap();
             led_two.set_low().unwrap();
             led_three.set_low().unwrap();
             led_four.set_low().unwrap();
        }
        delay.delay_us(100u8);
        column_one.set_low().unwrap();                         // set column 1 to low
        column_two.set_high().unwrap();                        // set column 2 to high
        if input_one.is_high().unwrap() {                      // input 1 high button 2
             led_two.set_high().unwrap();
             led_one.set_low().unwrap();
             led_three.set_low().unwrap();
             led_four.set_low().unwrap();
        } else if input_two.is_high().unwrap() {               // input 2 high button 7
             led_one.set_high().unwrap();
             led_two.set_high().unwrap();
             led_three.set_high().unwrap();
             led_four.set_low().unwrap();           
        } else {
             led_one.set_low().unwrap();
             led_two.set_low().unwrap();
             led_three.set_low().unwrap();
             led_four.set_low().unwrap();        
        }
        delay.delay_us(100u8);
        column_two.set_low().unwrap();                         // set column 2 to low
        column_three.set_high().unwrap();                      // set column 3 to high
        if input_one.is_high().unwrap() {                      // input 1 high button 3
             led_one.set_high().unwrap();
             led_two.set_high().unwrap();
             led_three.set_low().unwrap();
             led_four.set_low().unwrap();
        } else if input_two.is_high().unwrap() {               // input 2  button 8
             led_four.set_high().unwrap();
             led_one.set_low().unwrap();
             led_two.set_low().unwrap();
             led_three.set_low().unwrap();
        } else {
             led_one.set_low().unwrap();
             led_two.set_low().unwrap();
             led_three.set_low().unwrap();
             led_four.set_low().unwrap();    
        }
        delay.delay_us(100u8);
        column_three.set_low().unwrap();                       // set column 3 to low
        column_four.set_high().unwrap();                       // set column 4 to high
        if input_one.is_high().unwrap() {                      // input 1 high button 4
             led_three.set_high().unwrap();
             led_one.set_low().unwrap();
             led_two.set_low().unwrap();
             led_four.set_low().unwrap();
        } else if input_two.is_high().unwrap() {               // input 2 high button 9
             led_four.set_high().unwrap();
             led_one.set_high().unwrap();
             led_two.set_low().unwrap();
             led_three.set_low().unwrap();
        } else {
             led_one.set_low().unwrap();
             led_two.set_low().unwrap();
             led_three.set_low().unwrap();
             led_four.set_low().unwrap();    
        }
        delay.delay_us(100u8);
        column_four.set_low().unwrap();                        // set column 4 to low
        column_five.set_high().unwrap();                       // set column 5 to high
        if input_one.is_high().unwrap() {                      // input 1 high button 5
             led_one.set_high().unwrap();   
             led_three.set_high().unwrap();
             led_two.set_low().unwrap();
             led_four.set_low().unwrap();
        } else if input_two.is_high().unwrap() {               // input 2 high button 10
             led_two.set_high().unwrap();
             led_four.set_high().unwrap();
             led_one.set_low().unwrap();
             led_three.set_low().unwrap();
        } else {
             led_one.set_low().unwrap();
             led_two.set_low().unwrap();
             led_three.set_low().unwrap();
             led_four.set_low().unwrap();
        }   
        delay.delay_us(100u8);
        column_five.set_low().unwrap();                        // set column 5 to low
     }
}

//fn none_pressed() {
//    let mut peripherals = Peripherals::take().unwrap();
//    let core = CorePeripherals::take().unwrap();
//    let mut clocks = GenericClockController::with_external_32kosc(
//        peripherals.GCLK,
//        &mut peripherals.PM,
//        &mut peripherals.SYSCTRL,
//        &mut peripherals.NVMCTRL,
//    );
//    let mut pins = hal::Pins::new(peripherals.PORT);
//    let mut led_one = pins.d13.into_push_pull_output(&mut pins.port);
//    let mut led_two = pins.a5.into_push_pull_output(&mut pins.port);
//    let mut led_three = pins.d5.into_push_pull_output(&mut pins.port);
//    let mut led_four = pins.d6.into_push_pull_output(&mut pins.port);
//    led_one.set_low();
//    led_two.set_low();
//    led_three.set_low();
//    led_four.set_low();
//}

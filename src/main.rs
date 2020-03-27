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
    let mut led_one = pins.d13.into_push_pull_output(&mut pins.port);
    let mut led_two = pins.a5.into_push_pull_output(&mut pins.port);
    let mut led_three = pins.d5.into_push_pull_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let input_one = pins.d11.into_pull_down_input(&mut pins.port);
    let input_two = pins.d12.into_pull_down_input(&mut pins.port);

    loop { 
        column_two.set_high().unwrap(); 
        column_one.set_low().unwrap();                        // set column 1 to low
        if input_one.is_low().unwrap() {                      // input 1 low button 1
             led_one.set_high().unwrap();
             led_two.set_low().unwrap();
             led_three.set_low().unwrap();
        } else if input_two.is_low().unwrap() {               // input 2 low button 3
             led_three.set_low().unwrap();
             led_two.set_high().unwrap();
             led_one.set_high().unwrap();
        } else {
             led_one.set_low().unwrap();
             led_two.set_low().unwrap();
             led_three.set_low().unwrap();
        }
        delay.delay_us(100u8);
        column_one.set_high().unwrap();                         // set column 1 to high
        column_two.set_low().unwrap();                        // set column 2 to low
        if input_one.is_low().unwrap() {                      // input 1 low button 2
             led_two.set_high().unwrap();
             led_one.set_low().unwrap();
             led_three.set_low().unwrap();
        } else if input_two.is_low().unwrap() {               // input 2 low button 4
             led_one.set_low().unwrap();
             led_two.set_low().unwrap();
             led_three.set_high().unwrap();           
        } else {
             led_one.set_low().unwrap();
             led_two.set_low().unwrap();
             led_three.set_low().unwrap();        
        }
        delay.delay_us(100u8);
        column_two.set_high().unwrap();                         // set column 2 to high  
     }
}
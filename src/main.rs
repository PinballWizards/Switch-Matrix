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
use hal::adc::Adc;                          
use cortex_m_semihosting::hprintln;       

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
    let mut column_one = pins.a1.into_open_drain_output(&mut pins.port);
    let mut column_two = pins.a2.into_open_drain_output(&mut pins.port);
    let mut column_three = pins.a3.into_open_drain_output(&mut pins.port);
    let mut column_four = pins.a4.into_open_drain_output(&mut pins.port);
    let mut column_five = pins.d10.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut input_one = pins.d11.into_function_b(&mut pins.port);
    let mut input_two = pins.d12.into_function_b(&mut pins.port);

    loop {
       column_one.set_high().unwrap();     // set column 1 to high
       if input_one  {                     // input 1 high
            println!("Button 1 was pressed");
       } else if input_two {               // input 2 high
            println!("Button 6 was pressed");
       } 
       delay.delay_ms(200u8);
       column_one.set_low().unwrap();      // set column 1 to low
       column_two.set_high().unwrap();     // set column 2 to high 
       if input_one {                      // input 1 high
            println!("Button 2 was pressed");
       } else if input_two {               // input 2 high
            println!("Button 7 was pressed");
       }
       delay.delay_ms(200u8);
       column_two.set_low().unwrap();      // set column 2 to low
       column_three.set_high().unwrap();   // set column 3 to high
       if input_one {                      // input 1 high
            println!("Button 3 was pressed");
       } else if input_two {               // input 2 high
            println!("Button 8 was pressed");
       }
       delay.delay_ms(200u8);
       column_three.set_low().unwrap();    // set column 3 to low
       column_four.set_high().unwrap();    // set column 4 to high
       if input_one {                      // input 1 high
            println!("Button 4 was pressed");
       } else if input_two {               // input 2 high
            println!("Button 9 was pressed")'
       }
       delay.delay_ms(200u8);
       column_four.set_low().unwrap();     // set column 4 to low
       column_five.set_high().unwrap();    // set column 5 to high
       if input_one {                      // input 1 high
            println!("Button 5 was pressed");
       } else if input_two {               // input 2 high
            println!("Button 10 was pressed");
       }
       delay.delay_ms(200u8);
       column_five.set_low().unwrap();     // set column 5 to low
    }
}

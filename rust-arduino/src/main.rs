// main.rs

#![no_std]
#![no_main]
use panic_halt as _;
#[allow(unused_imports)]
use arduino_uno::hal::port::mode::Output;
#[allow(unused_imports)]
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::prelude::*;
use dht11::Dht11;
extern crate  ufmt;
extern crate nb;
use arduino_uno::i2cMaster;
use atmega328p_hal::{delay,clock};
use lcd::{Delay,Hardware};
// create a pcfdriver
struct Pcf8574 { 
pub dev:i2cMaster,
pub data:u8,

}
// implement traits for dis
imp Pcf8574 { 

    pub fn new(dev:i2cMaster) -> Self { 
        Pcf8574 { 
            dev,
            data:data: 0b0000_1000, // backlight on by default
        }
    }

    pub fn backlight(&mut self,on:bool) { 
        self.set_bit(2,on);
        self.apply();
    }


    fn set_bit(&mut self,offset:u8,bit:bool) { 
        if bit {
            self.data |= 1 << offset;
        } else {
            self.data &= !(1 << offset);
        }
    }

}

impl Hardware for Pcf8574 { 
    fn rs(&mut self, bit: bool) {
        self.set_bit(0, bit);
    }

    fn enable(&mut self, bit: bool) {
        self.set_bit(2, bit);
    }

    fn data(&mut self, bits: u8) {
        assert!(bits & 0xF0 == 0, "4-bit mode is required");
        self.data = (self.data & 0x0F) | (bits << 4);
    }

    fn apply(&mut self) {
        // No error handling.
        let _ = self.dev.smbus_write_byte(self.data);
    }


}
impl Delay for Pcf8574 { 
    fn delay_us(&mut self,delay:u16) { 
        arduino_uno::delay_ms(delay);
    }
}
#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();
     
    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);
    let mut delay = delay::Delay::<clock::MHz16>::new();
    let  sensor= pins.d2.into_tri_state (&mut pins.ddr);
    // initialize DHT data transfer

#[allow(unused_must_use)] 
    
  
    let mut serial = arduino_uno::Serial::new(
        peripherals.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );


  
    
   // create an instance of the sensor
   let mut dht11 = Dht11::new(sensor);
  
    
     ufmt::uwriteln!(&mut serial," Here the reading from sensors !\r").void_unwrap();
    
    loop {
           match dht11.perform_measurement(&mut delay) {
              Ok(readings) => { 
   // write result to serial 
   ufmt::uwriteln!(&mut serial, "{}", readings.temperature ).void_unwrap();
              },
              Err(_) => { 
           //   ufmt::uwriteln!(&mut serial,  "failed to read").void_unwrap();
              }
    
           }
           
           
           arduino_uno::delay_ms(1500);
         }
         
    }

   
/* fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i as u16);
    });
}
*/

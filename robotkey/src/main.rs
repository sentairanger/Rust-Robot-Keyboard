// Import libraries
extern crate termion;

use termion::event::Key;

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use rppal::gpio::Gpio;
use std::error::Error;

// Define pins
const PIN_1: u8 = 13;
const PIN_2: u8 = 21;
const PIN_3: u8 = 17;
const PIN_4: u8 = 27; 

fn main() -> Result<(), Box<dyn Error>> {
    // Connect the pins to the constants
    let mut pin1 = Gpio::new()?.get(PIN_1)?.into_output();
    let mut pin2 = Gpio::new()?.get(PIN_2)?.into_output();
    let mut pin3 = Gpio::new()?.get(PIN_3)?.into_output();
    let mut pin4 = Gpio::new()?.get(PIN_4)?.into_output();
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    // Initiate keyboard control
    write!(stdout,
           "{}{}q to exit. Type stuff, use alt, and so on.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();
    
     // Move the robot in a specified direction depending on the key pressed
    for c in stdin.keys() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 1),
               termion::clear::CurrentLine)
                .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Left => {
                pin1.set_low();
                pin2.set_high();
                pin3.set_high();
                pin4.set_low();
            }
            Key::Right => {
                pin1.set_high();
                pin2.set_low();
                pin3.set_low();
                pin4.set_high();
            }
            Key::Up => {
                pin1.set_high();
                pin2.set_low();
                pin3.set_high();
                pin4.set_low();
            }
            Key::Down => {
                pin1.set_low();
                pin2.set_high();
                pin3.set_low();
                pin4.set_high();
            }
            Key::Char(Enter) => {
                pin1.set_low();
                pin2.set_low();
                pin3.set_low();
                pin4.set_low();
            }
            _ => {}
        }
        stdout.flush().unwrap();
       }
        write!(stdout, "{}", termion::cursor::Show).unwrap();
    // This eliminates a major error so be sure to add this 
     Ok(())

   

   
}


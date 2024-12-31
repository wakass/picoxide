#![allow(unused)]
#![no_std]
#![no_main]

use crate::prelude::*;
use cortex_m_rt::entry;
use panic_halt as _; //Ensure we have a panic handler

mod error;
mod prelude;
mod utils;

#[entry]
fn main() -> ! {
    let somevar = 1;
    let start_dir = "./";
    loop {}
}

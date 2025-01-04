#![allow(unused)]
#![no_std]
#![no_main]

use crate::prelude::*;
use cortex_m_rt::entry;
use panic_halt as _; //Ensure we have a panic handler
use rp235x_hal as hal;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use core::fmt::Write;
use heapless::String;
use usbd_serial::SerialPort;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use rtt_target::{rprintln, rtt_init_print};

mod error;
mod prelude;
mod utils;

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[hal::entry]
fn main() -> ! {
    rtt_init_print!();

    let mut pac = hal::pac::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    //USB-serial init
    //
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USB,
        pac.USB_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Forall labs")
            .product("Serial port")
            .serial_number("1337")])
        .unwrap()
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    // Configure GPIO22 as an output (pin 25 not available on pico-w's)
    let mut led_pin = pins.gpio22.into_push_pull_output();
    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf[..]) {
            Ok(count) => match buf {
                s if s.starts_with(b"x") => {
                    let _ = serial.write(b"HI!");
                    rprintln!("Wrote to serial");
                }
                _ => {}
            },
            Err(UsbError::WouldBlock) => {} // No data received
            Err(err) => {}                  // An error occurred
        };

        //Blinking an LED is not compatible with a wanted response time of 10ms for USB
        //rprintln!("Hello ello");
        //led_pin.set_high().unwrap();
        //timer.delay_ms(500);
        //led_pin.set_low().unwrap();
        //timer.delay_ms(500);
    }
}

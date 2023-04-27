#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use rp_pico::entry;
use panic_halt as _;
use rp_pico::hal::pac;
use rp_pico::hal;
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;
use core::fmt::Write;
use heapless::String;

use embedded_cylinder::CylinerBuildU8;

#[derive(CylinerBuildU8)]
enum Test {
    MyEnum1,
    MyEnum2,
    MyEnum3,
    MyEnum4
}

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
        .ok()
        .unwrap();

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut serial_sent = false;
    loop {
        // A welcome message at the beginning
        if !serial_sent && timer.get_counter().ticks() >= 2_000_000 {
            serial_sent = true;
            let hash : HashMap<&Test, u8> = Test::describe();
            for (key, value) in &hash {
                let x = format!("{:?},{}\r\n",key,value);
                let _ = serial.write(x.as_bytes());
            }
        }

        // This has to be here to make the port available
        if usb_dev.poll(&mut [&mut serial]) {}
    }
}
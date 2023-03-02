#![no_std]
#![no_main]
#![feature(iter_intersperse)]

mod keyboard;

use core::panic::PanicInfo;

use keyboard::print_reports;
use rp_pico::{
    entry,
    hal::{clocks, pac::interrupt, usb, Clock, Watchdog},
    pac,
};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
    UsbError,
};
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};

#[panic_handler]
fn _panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

const USB_DELAY: u8 = 5;

static mut USB_DEVICE: Option<UsbDevice<usb::UsbBus>> = None;
static mut USB_BUS: Option<UsbBusAllocator<usb::UsbBus>> = None;
static mut USB_HID: Option<HIDClass<usb::UsbBus>> = None;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = clocks::init_clocks_and_plls(
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

    #[cfg(feature = "rp2040-e5")]
    {
        use rp_pico::{hal::Sio, Pins};

        let sio = Sio::new(pac.SIO);
        let _pins = Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );
    }

    // Setup the USB driver
    let usb_bus = UsbBusAllocator::new(usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    unsafe {
        // This is safe as interrupts haven't been started yet.
        USB_BUS = Some(usb_bus);
    }

    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // Set up the device class driver
    let usb_hid = HIDClass::new(bus_ref, KeyboardReport::desc(), USB_DELAY);

    unsafe {
        // This is safe as interrupts haven't been started yet.
        USB_HID = Some(usb_hid);
    }

    // Can you find out what device this is emulating?
    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x045E, 0x780))
        .serial_number("PWNED")
        .device_class(0x00)
        .build();

    unsafe {
        // This is safe as interrupts haven't been started yet.
        USB_DEVICE = Some(usb_dev);
    }

    // Enable the USB interrupt
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::USBCTRL_IRQ);
    }

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    loop {
        for report in print_reports("echo 'Hello, world!'\n") {
            push_report(report);
            delay.delay_ms(USB_DELAY as u32);
        }

        delay.delay_ms(1000);
    }
}

fn push_report(report: KeyboardReport) -> Result<usize, UsbError> {
    critical_section::with(|_| unsafe { USB_HID.as_mut().map(|hid| hid.push_input(&report)) })
        .unwrap()
}

/// This function is called whenever the USB hardware generates an inturrupt request.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();

    usb_dev.poll(&mut [usb_hid]);
}

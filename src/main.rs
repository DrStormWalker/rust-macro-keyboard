#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rp_pico::{
    entry,
    hal::{clocks, pac::interrupt, usb, Clock, Sio, Watchdog},
    pac, Pins,
};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};

#[panic_handler]
fn _panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

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
    let usb_hid = HIDClass::new(bus_ref, KeyboardReport::desc(), 10);

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
        // Keycodes can be found [here](https://usb.org/sites/default/files/hut1_4.pdf)
        // Up to 6 keycodes can be placed in one report, and will be enter one after the other
        // Any duplicate keycodes will be ignored, so to send two `0`s in a row
        // two reports must be sent with a reset report between them.
        let report = KeyboardReport {
            modifier: 0x0,
            reserved: 0,
            leds: 0x0,
            //            H     E     L
            keycodes: [0x0B, 0x08, 0x0F, 0x00, 0x00, 0x00],
        };

        let _ = critical_section::with(|_| unsafe {
            USB_HID.as_mut().map(|hid| hid.push_input(&report))
        })
        .unwrap();

        delay.delay_ms(10);

        // This is a reset report, it clears all held keys, telling the host they are no longer
        // pressed
        let report = KeyboardReport {
            modifier: 0x0,
            reserved: 0,
            leds: 0x0,
            keycodes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        };

        let _ = critical_section::with(|_| unsafe {
            USB_HID.as_mut().map(|hid| hid.push_input(&report))
        })
        .unwrap();

        delay.delay_ms(10);

        let report = KeyboardReport {
            modifier: 0x0,
            reserved: 0,
            leds: 0x0,
            //            L     0     .  space
            keycodes: [0x0F, 0x12, 0x37, 0x2C, 0x00, 0x00],
        };

        let _ = critical_section::with(|_| unsafe {
            USB_HID.as_mut().map(|hid| hid.push_input(&report))
        })
        .unwrap();

        delay.delay_ms(10);

        let report = KeyboardReport {
            modifier: 0x0,
            reserved: 0,
            leds: 0x0,
            keycodes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        };

        let _ = critical_section::with(|_| unsafe {
            USB_HID.as_mut().map(|hid| hid.push_input(&report))
        })
        .unwrap();

        delay.delay_ms(1000);
    }
}

/// This function is called whenever the USB hardware generates an inturrupt request.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();

    usb_dev.poll(&mut [usb_hid]);
}

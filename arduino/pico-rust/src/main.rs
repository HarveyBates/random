// Example blinky from 
// https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal/examples/blinky.rs
// Couldn't get it to work, there needs to be this in the .cargo/config.toml:
//
// [target.'cfg(all(target_arch = "arm", target_os = "none"))']
// runner = "elf2uf2-rs -d"
// [build]
// target = "thumbv6m-none-eabi"
//
// However, the pico has to be mounted for it to find and flash

// Disable standardlib and main
#![no_main]
#![no_std]

// Define entry point for program 
use cortex_m_rt::entry;

// Halt the program on panic
use panic_halt as _;

// Alias for HAL
use rp2040_hal as hal;

// Access Peripherial Access Crate (i2c, SPI etc.)
use hal::pac;

// Import some traits
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use rp2040_hal::clocks::Clock;

// Linker setup
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

// Crystal freqency (12 MHz)
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

// Define the entry point function
#[entry]
fn main() -> ! {
    // Define peripherials
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Setup watchdog (required by clock)
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, 
        clocks.system_clock.freq().integer());

    // Single cycle I/O block controls 
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure blinking 
    let mut led_pin = pins.gpio25.into_push_pull_output();
    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(1000);
        led_pin.set_low().unwrap();
        delay.delay_ms(1000);
    }
}

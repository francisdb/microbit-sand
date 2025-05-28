#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{OutputPin, PinState};
use microbit::Board;
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use nrf52833_hal as hal;
use nrf52833_hal::gpio::Level;
// Use the panic handler that halts the system
use panic_halt as _;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");

    //blink_hal();

    //blink_bsp();

    // blink_bsp_display();

    blink_bsp_display_random();
}

fn blink_bsp() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut col1 = board.display_pins.col1.into_push_pull_output(Level::Low);
    let mut col2 = board.display_pins.col2.into_push_pull_output(Level::Low);
    let mut col3 = board.display_pins.col3.into_push_pull_output(Level::Low);
    let mut col4 = board.display_pins.col4.into_push_pull_output(Level::Low);
    let mut col5 = board.display_pins.col5.into_push_pull_output(Level::Low);
    let mut row1 = board.display_pins.row1.into_push_pull_output(Level::Low);
    let mut row2 = board.display_pins.row2.into_push_pull_output(Level::Low);
    let mut row3 = board.display_pins.row3.into_push_pull_output(Level::Low);
    let mut row4 = board.display_pins.row4.into_push_pull_output(Level::Low);
    let mut row5 = board.display_pins.row5.into_push_pull_output(Level::Low);

    loop {
        rprintln!("Led on");
        // col1.set_high().unwrap();
        // col2.set_high().unwrap();
        // col3.set_high().unwrap();
        // col4.set_high().unwrap();
        // col5.set_high().unwrap();
        row1.set_high().unwrap();
        row2.set_high().unwrap();
        row3.set_high().unwrap();
        row4.set_high().unwrap();
        row5.set_high().unwrap();
        timer.delay_ms(1_000);
        rprintln!("Led off");
        // col1.set_low().unwrap();
        // col2.set_low().unwrap();
        // col3.set_low().unwrap();
        // col4.set_low().unwrap();
        // col5.set_low().unwrap();
        row1.set_low().unwrap();
        row2.set_low().unwrap();
        row3.set_low().unwrap();
        row4.set_low().unwrap();
        row5.set_low().unwrap();
        timer.delay_ms(1_000)
    }
}

fn random_matrix(rng: &mut SmallRng) -> [[u8; 5]; 5] {
    let mut matrix = [[0; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            matrix[i][j] = rng.random_range(0..2); // Randomly set to 0 or 1
        }
    }
    matrix
}

fn blink_bsp_display_random() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let seed = [0; 32]; // Use a fixed seed for reproducibility
    let mut rng = SmallRng::from_seed(seed);

    let mut left_top: [[u8; 5]; 5] = random_matrix(&mut rng);

    loop {
        display.show(&mut timer, left_top, 100);
        left_top = random_matrix(&mut rng);
    }
}

fn blink_bsp_display() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let left_top: [[u8; 5]; 5] = [
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let empty: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    loop {
        rprintln!("Led on");
        display.show(&mut timer, left_top, 1_000);
        rprintln!("Led off");
        display.show(&mut timer, empty, 1_000);
    }
}

fn blink_hal() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let _col1 = port0.p0_28.into_push_pull_output(Level::Low);
    let mut row1 = port0.p0_21.into_push_pull_output(Level::Low);

    let mut is_on = true;
    loop {
        rprintln!("Led {}", if is_on { "on" } else { "off" });
        row1.set_state(PinState::from(is_on)).unwrap();
        for _ in 0..200_000 {
            nop();
        }
        is_on = !is_on;
    }
}
// #[panic_handler]
// fn panic_handler(_: &core::panic::PanicInfo) -> ! {
//     // Handle panic, e.g., by entering an infinite loop
//     loop {}
// }

#![no_main]
#![no_std]

mod counter;
mod led;
mod serial;
use cortex_m_rt::entry;
use counter::IncrementalCounter;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
// use defmt_rtt as _;
use microbit::{
    display::blocking::Display,
    hal::{
        gpio::{Level, OpenDrainConfig},
        prelude::*,
        saadc::SaadcConfig,
        Saadc, Timer,
    },
    Board,
};
use serial::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut saadc = Saadc::new(board.SAADC, SaadcConfig::default());

    let mut mic_in = board.microphone_pins.mic_in.into_floating_input();
    board
        .microphone_pins
        .mic_run
        .into_open_drain_output(OpenDrainConfig::Disconnect0HighDrive1, Level::High);

    let mut count: u64 = 0;
    let mut sum: u64 = 0;
    let mut max_value: u16 = 0;
    let mut serial = UartePort::create(board.UARTE0, board.uart);
    display.show(&mut timer, led::A, 1000);

    loop {
        serial.write("Staring loop");
        let mic_value = saadc
            .read(&mut mic_in)
            .map_err(|_e| serial.write("could not read from microphone\r\t"))
            .expect("could not read from microphone") as u16;

        max_value += max_value.max(mic_value);
        sum += mic_value as u64;
        count += 1;
        serial.stats(max_value, sum, count);

        if count.every(10) {
            let avg = (sum / count) as u16;
            let image = [
                [if max_value > avg + 100 { 1 } else { 0 }; 5],
                [if max_value > avg + 80 { 1 } else { 0 }; 5],
                [if max_value > avg + 60 { 1 } else { 0 }; 5],
                [if max_value > avg + 40 { 1 } else { 0 }; 5],
                [if max_value > avg + 20 { 1 } else { 0 }; 5],
            ];

            display.show(&mut timer, image, 10);
            max_value = 0;
        }
    }
}

mod test {
    //
}

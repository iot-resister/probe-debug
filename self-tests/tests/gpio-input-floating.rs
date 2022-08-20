#![no_std]
#![no_main]

use defmt_rtt as _;    // transport layer for defmt logs
use nrf52840_hal as _; // the HAL we'll test
use panic_probe as _;  // panicking behavior

#[defmt_test::tests]
mod tests {
    use defmt::assert;

    #[test]
    fn always_passes() {
        assert!(true);
    }
}

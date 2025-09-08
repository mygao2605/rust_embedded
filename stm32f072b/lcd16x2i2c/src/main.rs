#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f0xx_hal as hal;
use hal::{
    pac,
    prelude::*,
    i2c::I2c,
};
use cortex_m::Peripherals;
use hd44780_driver::HD44780;

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    // Clock config
    let mut rcc = dp.RCC.configure().freeze(&mut dp.FLASH);

    // GPIOB
    let gpio = dp.GPIOB.split(&mut rcc);

    let cs = unsafe { &cortex_m::interrupt::CriticalSection::new() };

    // PB6 = SCL, PB7 = SDA
    let scl = gpio.pb6.into_alternate_af1(cs).set_open_drain(cs);
    let sda = gpio.pb7.into_alternate_af1(cs).set_open_drain(cs);

    // I2C1
    let i2c = I2c::i2c1(
        dp.I2C1,
        (scl, sda),
        100.khz(),
        &mut rcc,
    );

    // Delay
    let mut delay = hal::delay::Delay::new(cp.SYST, &rcc);

    // PCF8574 địa chỉ mặc định 0x27 (hoặc 0x3F tuỳ module)
    // Không cần I2CBus, dùng trực tiếp I2C + addr
    let mut lcd = HD44780::new_i2c(i2c, 0x27, &mut delay).unwrap();

    lcd.reset(&mut delay).unwrap();
    lcd.clear(&mut delay).unwrap();
    lcd.write_str("Hello Rust!", &mut delay).unwrap();


    loop {}
}

#![no_std]
#![no_main]

use panic_halt as _;  // Panic handler
use cortex_m_rt::entry;
use stm32f0xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Khai báo `dp` là mutable
    let mut dp = pac::Peripherals::take().unwrap(); // lay tao bo ngoai vi cua chip(peripheral access crate - PAC such as GPIO, RCC, USART, etc.)
    let mut rcc = dp.RCC.configure().freeze(&mut dp.FLASH);

    // Cấu hình GPIO (sử dụng CriticalSection nếu HAL yêu cầu)
    let gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = cortex_m::interrupt::free(|cs| gpioc.pc6.into_push_pull_output(cs));
    let mut led2 = cortex_m::interrupt::free(|cs| gpioc.pc7.into_push_pull_output(cs));
    let mut led3: stm32f0xx_hal::gpio::gpioc::PC8<stm32f0xx_hal::gpio::Output<stm32f0xx_hal::gpio::PushPull>> = cortex_m::interrupt::free(|cs| gpioc.pc8.into_push_pull_output(cs));
    let mut led4: stm32f0xx_hal::gpio::gpioc::PC9<stm32f0xx_hal::gpio::Output<stm32f0xx_hal::gpio::PushPull>> = cortex_m::interrupt::free(|cs| gpioc.pc9.into_push_pull_output(cs));
    loop {
        led.set_high().ok();
        led2.set_high().ok();
        led3.set_high().ok();
        led4.set_high().ok();
        cortex_m::asm::delay(1_000_000);  // Delay ~1s (tùy chỉnh tần số CPU)
        led.set_low().ok();
        led2.set_low().ok();
        cortex_m::asm::delay(1_000_000);
    }
}
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::interrupt;
use panic_halt as _;

use stm32f0xx_hal::{
    pac,
    prelude::*,
    spi::Spi,
    gpio::{Output, PushPull, gpiob::{PB0, PB1}},
    delay::Delay,
};

// thêm dòng này:
use embedded_hal::spi::MODE_0;


use st7735_lcd::ST7735;
use embedded_graphics::{
    prelude::*,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    text::Text,
};

#[entry]
fn main() -> ! {
    // lấy peripherals
    let  mut dp = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    // clock & delay
    let mut rcc = dp.RCC.configure().freeze(&mut dp.FLASH);
    let mut delay = Delay::new(cp.SYST, &rcc);

    // split GPIO
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    interrupt::free(|cs| {
        // SPI pins
        let sck  = gpioa.pa5.into_alternate_af0(cs); // SCK
        let miso = gpioa.pa6.into_alternate_af0(cs); // MISO
        let mosi = gpioa.pa7.into_alternate_af0(cs); // MOSI

        // CS pin giả (ko cần thiết nhưng tạo PA4 để truyền vào)
        let cs_pin = gpioa.pa4.into_push_pull_output(cs);

        // DC và RST
        let dc: PB1<Output<PushPull>>  = gpiob.pb1.into_push_pull_output(cs);
        let mut rst: PB0<Output<PushPull>> = gpiob.pb0.into_push_pull_output(cs);

        // init SPI
        let spi = Spi::spi1(
            dp.SPI1,
            (sck, miso, mosi),
            MODE_0,
            8.mhz(),
            &mut rcc,
        );

        // reset LCD
        rst.set_low().ok();
        delay.delay_ms(50u16);
        rst.set_high().ok();
        delay.delay_ms(50u16);

        // tạo driver
        let mut disp = ST7735::new(spi, cs_pin, dc, true, false, 160, 128);

        disp.init(&mut delay).unwrap();
        disp.set_orientation(&st7735_lcd::Orientation::Portrait).unwrap();

        // clear màn hình
        disp.clear(Rgb565::BLACK).unwrap();

        // hiển thị chữ
        let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
        Text::new("Hello Rust!", Point::new(10, 30), style)
            .draw(&mut disp)
            .unwrap();

        Text::new("STM32F072 + ST7735", Point::new(10, 50), style)
            .draw(&mut disp)
            .unwrap();
    });

    loop {
        // main loop rỗng
    }
}

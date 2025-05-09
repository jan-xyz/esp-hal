//! UART Test

//% CHIPS: esp32 esp32c2 esp32c3 esp32c6 esp32h2 esp32s2 esp32s3
//% FEATURES: unstable embassy

#![no_std]
#![no_main]

use esp_hal::{
    Async,
    uart::{self, Uart},
};
use hil_test as _;

struct Context {
    uart: Uart<'static, Async>,
}

#[cfg(test)]
#[embedded_test::tests(default_timeout = 3, executor = hil_test::Executor::new())]
mod tests {
    use super::*;

    #[init]
    async fn init() -> Context {
        let peripherals = esp_hal::init(esp_hal::Config::default());

        let (rx, tx) = hil_test::common_test_pins!(peripherals);

        let uart = Uart::new(peripherals.UART0, uart::Config::default())
            .unwrap()
            .with_tx(tx)
            .with_rx(rx)
            .into_async();

        Context { uart }
    }

    #[test]
    async fn test_send_receive(mut ctx: Context) {
        const SEND: &[u8] = b"Hello ESP32";
        let mut buf = [0u8; SEND.len()];

        ctx.uart.flush_async().await.unwrap();
        ctx.uart.write_async(&SEND).await.unwrap();
        ctx.uart.flush_async().await.unwrap();

        ctx.uart.read_async(&mut buf[..]).await.unwrap();
        assert_eq!(&buf[..], SEND);
    }
}

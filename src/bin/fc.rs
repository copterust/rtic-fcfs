#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use rtic_fcfs as _;

#[rtic::app(
    device = stm32f3xx_hal::pac,
    dispatchers = [CAN_RX1]
)]
mod app {
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");
        task1::spawn().ok();

        (Shared {}, Local {})
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    #[task(priority = 1)]
    async fn task1(_cx: task1::Context) {
        defmt::info!("Hello from task1!");
    }
}

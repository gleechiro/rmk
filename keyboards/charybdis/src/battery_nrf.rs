//! VDDH battery reader for a genuine nice!nano module.
//!
//! nice!nano's own PMIC feeds the battery voltage into VDDH, so no external
//! divider or extra GPIO pin is needed (unlike Ergohaven's own PCBs, which
//! read a dedicated AIN pin through their own divider resistors).
//!
//! Stock RMK's `[ble].battery_adc_pin = "vddh"` path is not used: its NrfAdc
//! init calls `calibrate().await`, which can hang under the BLE SoftDevice
//! (documented and worked around the same way by Ergohaven's own boards).
//! This reader skips calibrate, times out samples, and re-publishes
//! BatteryStatusEvent every 2s so the split loop can forward it.

use embassy_futures::select::select;
use embassy_nrf::interrupt;
use embassy_nrf::interrupt::InterruptExt;
use embassy_nrf::peripherals::SAADC;
use embassy_nrf::saadc::{self, Input as _, Saadc};
use embassy_nrf::{bind_interrupts, Peri};
use embassy_time::{with_timeout, Duration, Timer};
use rmk::core_traits::Runnable;
use rmk::event::{EventSubscriber, PeripheralBatteryRefreshEvent, SubscribableEvent};
use rmk::input_device::battery::publish_battery_status;
use rmk::processor::Processor;
use rmk::types::battery::{BatteryStatus, ChargeState};

bind_interrupts!(struct SaadcIrqs {
    SAADC => saadc::InterruptHandler;
});

// The nRF52840 SAADC reads VDDH through an internal /5 divider (see
// rmk-macro's own `battery_adc_pin = "vddh"` codegen), so the same divider
// applies here regardless of the PCB's actual battery-voltage range.
const MEASURED: i32 = 1;
const TOTAL: i32 = 5;

fn percent(val: u16) -> u8 {
    let val = val as i32;
    if val > 4755 * MEASURED / TOTAL {
        100
    } else if val < 4055 * MEASURED / TOTAL {
        0
    } else {
        ((val * TOTAL / MEASURED - 4055) / 7) as u8
    }
}

pub struct SplitBattery {
    saadc: Saadc<'static, 1>,
}

impl SplitBattery {
    pub fn new(saadc: Peri<'static, SAADC>) -> Self {
        interrupt::SAADC.set_priority(interrupt::Priority::P3);
        let channel = saadc::ChannelConfig::single_ended(saadc::VddhDiv5Input.degrade_saadc());
        Self {
            saadc: Saadc::new(saadc, SaadcIrqs, saadc::Config::default(), [channel]),
        }
    }

    async fn publish_sample(&mut self) {
        let mut buf = [0i16; 1];
        let level = match with_timeout(Duration::from_millis(200), self.saadc.sample(&mut buf)).await {
            Ok(()) => {
                let raw = if buf[0] < 0 { 0 } else { buf[0] as u16 };
                percent(raw)
            }
            Err(_) => 0,
        };
        publish_battery_status(BatteryStatus::Available {
            charge_state: ChargeState::Unknown,
            level: Some(level),
        });
    }
}

struct NeverSub;
pub struct NeverEvent;

impl EventSubscriber for NeverSub {
    type Event = NeverEvent;
    async fn next_event(&mut self) -> NeverEvent {
        core::future::pending().await
    }
}

impl Runnable for SplitBattery {
    async fn run(&mut self) -> ! {
        Timer::after(Duration::from_millis(1000)).await;
        let mut refresh_sub = PeripheralBatteryRefreshEvent::subscriber();
        loop {
            self.publish_sample().await;
            let _ = select(Timer::after(Duration::from_secs(2)), refresh_sub.next_event()).await;
        }
    }
}

impl Processor for SplitBattery {
    type Event = NeverEvent;
    fn subscriber() -> impl EventSubscriber<Event = NeverEvent> {
        NeverSub
    }
    async fn process(&mut self, _: NeverEvent) {}
    async fn process_loop(&mut self) -> ! {
        self.run().await
    }
}

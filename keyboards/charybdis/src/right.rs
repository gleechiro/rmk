#![no_main]
#![no_std]

mod battery_nrf;
#[path = "../../common/velvet_pointing.rs"]
mod velvet_pointing;

use rmk::macros::rmk_peripheral;

#[rmk_peripheral(id = 1)]
mod keyboard_peripheral {
    #[register_processor(event)]
    fn battery() -> crate::battery_nrf::SplitBattery {
        crate::battery_nrf::SplitBattery::new(p.SAADC)
    }

    #[register_processor(event)]
    fn pointing_settings() -> crate::velvet_pointing::VelvetPointingSettingsSync {
        crate::velvet_pointing::VelvetPointingSettingsSync::new()
    }
}

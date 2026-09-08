#![no_main]
#![no_std]

#[path = "../../common/default_layer_names.rs"]
mod default_layer_names;
#[path = "../../common/layer_names.rs"]
mod layer_names;
#[path = "../../common/velvet_pointing.rs"]
mod velvet_pointing;
#[path = "../../common/velvet_device_settings.rs"]
mod velvet_device_settings;

const DEFAULT_LAYER_NAMES: [&str; 16] = default_layer_names::STANDARD_WITH_MOUSE;

use rmk::macros::rmk_central;

#[rmk_central]
mod keyboard_central {
    #[register_processor(event)]
    fn pointing_mode() -> crate::velvet_pointing::VelvetPointingMode {
        crate::velvet_pointing::VelvetPointingMode::new()
    }

    #[register_processor(event)]
    fn settings_broadcast() -> crate::velvet_device_settings::VelvetSettingsBroadcast {
        crate::velvet_device_settings::VelvetSettingsBroadcast::new()
    }

    #[register_processor(poll)]
    fn ergohaven_user_keys() -> ::rmk::processor::builtin::ergohaven::ErgohavenUserKeys {
        ::rmk::processor::builtin::ergohaven::ErgohavenUserKeys::new()
    }
}

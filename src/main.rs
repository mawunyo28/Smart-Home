use std::sync::Arc;

use smart_home::smart_home::smart_device::{self, SmartDevice, SmartLight, SmartLightSpecs};

fn main() {
    let smart_light = SmartLight::MawunyoLamp(
        SmartDevice {
            name: String::from("Google TV"),
            model_number: String::from("GO293XMD"),
            device_description: String::from("Light"),
        },
        SmartLightSpecs {
            has_mobile_app: true,
            has_voice_control: true,
            has_remote_contorl: true,
            light_intensity: 203.0,
            light_levels: 9,
            current_light_level: 2,
        },
    );

    smart_light.power_on();

}

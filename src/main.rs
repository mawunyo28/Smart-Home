

use smart_home::smart_home::smart_device::{self, Device, SmartLight, SmartLightKind, SmartLightSpecs, SmartLightType};

fn main() {
    let smart_light = SmartLight {
        name: String::from("Mawunyo Lights"),
        model_number: String::from("(*302>023"),
        device_description: String::from("A smart light to flourish you ceilings"),
        specs: SmartLightSpecs {
            has_voice_control: true,
            has_remote_contorl: false,
            has_mobile_app: true,
            light_intensity: 200.20e2,
            light_levels: 23,
            current_light_level: 0,
        },
        smart_light_type: SmartLightType::Ceiling,
        smart_light_kind: SmartLightKind::MawunyoLamp,
    };

    smart_light.power_on();
    smart_light.print_device_info();

    

}

pub mod smart_device {

    use std::{thread::sleep, time::Duration};



    #[derive(Debug)]
    pub struct SmartDevice {
        name: String,
        model_number: String,
        device_description: String,
    }

    impl SmartDevice {
        pub fn power_on(&self) {
            println!(
                "Smart {} {} is powering on ......",
                self.device_description, self.name
            );
            sleep(Duration::from_secs(1));

            println!("Loading....");
            sleep(Duration::from_secs(3));

            println!("Smart {} {} is Ready!", self.device_description, self.name);
        }

        pub fn power_off(&self) {
            println!(
                "Smart {} {} is turning off.....",
                self.device_description, &self.name
            );
            sleep(Duration::from_secs(1));
        }

        pub fn from(name: String, model_number: String, device_description: String) -> SmartDevice {
            let smart_dev = SmartDevice {
                name,
                device_description,
                model_number,
            };

            fn init(device: &SmartDevice) {
                device.power_on();
            }

            init(&smart_dev);

            return smart_dev;
            
        }
    }

    struct SmartLightSpecs {
        has_voice_control: bool,
        has_remote_contorl: bool,
        has_mobile_app: bool,

        light_intensity: f64,
        light_levels: u8,
        current_light_level: u8,
    }

    enum SmartLight {
        GoogleLight(SmartDevice, SmartLightSpecs),
        AppleLight(SmartDevice, SmartLightSpecs),
        SamsungLight(SmartDevice, SmartLightSpecs),
        MawunyoLamp(SmartDevice, SmartLightSpecs),
    }

    impl SmartLight {
        fn power_on(&self) {
            match self {
                SmartLight::GoogleLight(smart_device, light_specs) => smart_device.power_on(),
                SmartLight::AppleLight(smart_device, light_specs) => smart_device.power_on(),
                SmartLight::SamsungLight(smart_device, light_specs) => smart_device.power_on(),
                SmartLight::MawunyoLamp(smart_device, light_specs) => smart_device.power_on(),
            }
        }

        fn power_off(&self) {
            match self {
                SmartLight::GoogleLight(smart_device, light_specs) => smart_device.power_off(),
                SmartLight::AppleLight(smart_device, light_specs) => smart_device.power_off(),
                SmartLight::SamsungLight(smart_device, light_specs) => smart_device.power_off(),
                SmartLight::MawunyoLamp(smart_device, light_specs) => smart_device.power_off(),
            }
        }
    }

    impl SmartLight {}

    enum SmartTV {}

    enum SmartTheatre {}

    enum SmartFridge {}
}

use crate::smart_home::smart_device::{Device, Specs};

#[derive(Debug)]
pub enum SmartLightType {
    Ceiling,
    Strip { lenght: f64 },
    Plug,
    Clip,
    DeskLamp,
    Lamp,
}

impl SmartLightType {
    fn get_type(&self) -> String {
        match &self {
            SmartLightType::Ceiling => String::new(),
            SmartLightType::Strip { lenght: length } => format!("Strip (length: {})", length),
            SmartLightType::Plug => String::new(),
            SmartLightType::Clip => String::new(),
            SmartLightType::DeskLamp => String::new(),
            SmartLightType::Lamp => String::new(),
        }
    }
}

impl SmartLightType {
    fn default() -> SmartLightType {
        SmartLightType::Ceiling
    }
}

#[derive(Debug)]
pub enum SmartLightKind {
    GoogleLight,
    MawunyoLamp,
    AppleSpark,
    OkRoverRGBStrips,
    SamsungPlayLights,
}

impl SmartLightKind {
    fn default() -> SmartLightKind {
        SmartLightKind::MawunyoLamp
    }
}

#[derive(Debug)]
pub struct SmartLight<'a> {
    pub name: &'a str,
    pub model_number: &'a str,
    pub device_description: &'a str,
    pub specs: SmartLightSpecs,
    pub smart_light_type: SmartLightType,
    pub smart_light_kind: SmartLightKind,
}

#[derive(Debug)]
pub struct SmartLightSpecs {
    pub has_voice_control: bool,
    pub has_remote_control: bool,
    pub has_mobile_app: bool,

    pub light_intensity: f64,
    pub light_levels: u8,
    pub current_light_level: u8,
}

impl SmartLight<'_> {
    fn new() -> SmartLight<'static> {
        SmartLight {
            name: "",
            model_number: "",
            device_description: "",
            specs: SmartLightSpecs::new(),
            smart_light_type: SmartLightType::default(),
            smart_light_kind: SmartLightKind::default(),
        }
    }

    fn new_from_parts<'a>(
        name: &'a str,
        model_number: &'a str,
        device_description: &'a str,
        mut specs: SmartLightSpecs,
        smart_light_type: SmartLightType,
        smart_light_kind: SmartLightKind,
    ) -> SmartLight<'a> {
        SmartLight {
            name,
            model_number,
            device_description,
            specs: {
                if specs.current_light_level > specs.light_levels {
                    specs.current_light_level = specs.light_levels;
                    specs
                }
                else {
                    specs
                }
            },
            smart_light_kind,
            smart_light_type,
        }
    }
}

impl SmartLightSpecs {
    fn new() -> SmartLightSpecs {
        SmartLightSpecs {
            has_voice_control: false,
            has_remote_control: false,
            has_mobile_app: false,
            light_intensity: 0.0,
            light_levels: 0,
            current_light_level: 0,
        }
    }

    fn new_from_parts(
        has_voice_control: bool,
        has_remote_control: bool,
        has_mobile_app: bool,
        light_intensity: f64,
        light_levels: u8,
        current_light_level: u8

    ) -> SmartLightSpecs {
        SmartLightSpecs {
            has_voice_control,
            has_remote_control,
            has_mobile_app,
            light_intensity,
            light_levels,
            current_light_level,
        }
    }
}

impl Device for SmartLight<'_> {
    fn power_on(&self) {
        todo!()
    }

    fn power_off(&self) {
        todo!()
    }

    fn get_device_info(&self) -> String {
        format!(
            "Device: {}, model number: {}, device description: {}, type: {:?}, kind {:?}",
            self.name,
            self.model_number,
            self.device_description,
            self.smart_light_type,
            self.smart_light_kind
        )
    }

    fn get_name(&self) -> String {
        format!("{}", &self.name)
    }

    fn get_device_description(&self) -> String {
        format!("{}", &self.device_description)
    }

    fn get_device_model_number(&self) -> String {
        format!("{}", &self.model_number)
    }
}

impl Specs for SmartLightSpecs {
    fn get_specs(&self) -> &SmartLightSpecs {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const fn get_test_smart_light_specs() -> SmartLightSpecs {
        SmartLightSpecs {
            has_voice_control: true,
            has_remote_control: true,
            has_mobile_app: true,
            light_intensity: 30.0,
            light_levels: 2,
            current_light_level: 3, // Validate this
        }
    }
    const fn get_test_smart_light() -> SmartLight<'static> {
        SmartLight {
            name: "Mawunyo Light",
            model_number: "MAW23",
            device_description: "RGB lights for your ceiling",
            specs: get_test_smart_light_specs(),
            smart_light_type: SmartLightType::Ceiling,
            smart_light_kind: SmartLightKind::MawunyoLamp,
        }
    }

    const TEST_DEVICE: SmartLight = get_test_smart_light();

    #[test]
    fn smart_light_get_name() {
        let expected_device_name = "Mawunyo Light".to_string();

        let test_device_name = TEST_DEVICE.get_name();

        assert_eq!(expected_device_name, test_device_name);
    }

    #[test]
    fn smart_light_get_device_description() {
        let expected_device_description = "RGB lights for your ceiling";

        let test_device_description = TEST_DEVICE.get_device_description();

        assert_eq!(expected_device_description, test_device_description);
    }

    #[test]
    fn smart_light_get_model_number() {
        let expected_device_model_number = "MAW23";

        let test_device_model_number = TEST_DEVICE.get_device_model_number();

        assert_eq!(expected_device_model_number, test_device_model_number);
    }
}

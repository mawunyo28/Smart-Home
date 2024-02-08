use crate::smart_home::smart_device::{Device, Specs};

#[derive(Debug)]
pub struct SmartLightSpecs {
    pub has_voice_control: bool,
    pub has_remote_contorl: bool,
    pub has_mobile_app: bool,

    pub light_intensity: f64,
    pub light_levels: u8,
    pub current_light_level: u8,
}

#[derive(Debug)]
pub enum SmartLightType {
    Ceiling,
    Strip { lenght: f64 },
    Plug,
    Clip,
    DeskLamp,
    Lamp,
}

#[derive(Debug)]
pub enum SmartLightKind {
    GoogleLight,
    MawunyoLamp,
    AppleSpark,
    OkRoverRGBStrips,
    SamsungPlayLights,
}

#[derive(Debug)]
pub struct SmartLight {
    pub name: String,
    pub model_number: String,
    pub device_description: String,
    pub specs: SmartLightSpecs,
    pub smart_light_type: SmartLightType,
    pub smart_light_kind: SmartLightKind,
}

impl Device for SmartLight {
    fn print_device_info(&self) {
        println!("Device {:#?}\nSpecs {:#?}", &self, &self.specs);
    }

    fn print_name(&self) {
        println!("Smart Light name: {}", &self.name);
    }

    fn device_description(&self) {
        println!("Smart Light description: {}", &self.device_description)
    }

    fn device_model_number(&self) {
        println!("Smart Light Description: {}", &self.device_description);
    }
}

impl Specs for SmartLightSpecs {
    fn getSpecs(&self) {
        println!("Smart Light's specs {:#?}", &self)
    }
}
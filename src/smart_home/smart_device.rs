use crate::smart_home::smart_device::smart_light::SmartLightSpecs;

pub trait Device {
    fn power_on(&self);

    fn power_off(&self);

    fn get_device_info(&self) -> String;

    fn get_name(&self) -> String;

    fn get_device_description(&self) -> String;

    fn get_device_model_number(&self) -> String;
}

pub trait Specs {
    fn get_specs(&self) -> &SmartLightSpecs;
}


pub mod smart_light;
pub mod smart_tv;
pub mod smart_theatre;

pub mod smart_fridge;



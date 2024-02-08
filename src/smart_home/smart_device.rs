use std::fmt::Debug;
use std::{thread::sleep, time::Duration};

pub trait Device {
    fn power_on(&self) {
        println!("Smart Device is powering on ......",);
        sleep(Duration::from_secs(1));

        println!("Loading....");
        sleep(Duration::from_secs(3));

        println!("Smart Device is Ready!");
    }

    fn power_off(&self) {
        println!("Smart Device is turning off.....",);
        sleep(Duration::from_secs(1));
        println!("Byebye....");
    }

    fn print_device_info(&self);

    fn print_name(&self);

    fn device_description(&self);

    fn device_model_number(&self);
}

pub trait Specs {
    fn getSpecs(&self);
}


pub mod smart_light;
pub mod smart_tv;
pub mod smart_theatre;

pub mod smart_fridge;



extern crate winapi;

use winapi::um::objbase::*;
use winapi::um::mmdeviceapi::*;
use winapi::um::endpointvolume::*;
use winapi::shared::*;
use winapi::Interface;

fn get_device_enumerator() -> *mut IMMDeviceEnumerator {
    let cls_mm_device_enum : guiddef::GUID = CLSID_MMDeviceEnumerator;
    let iid_imm_device_enumerator = IMMDeviceEnumerator::uuidof();

    let mut device_enumerator : *mut IMMDeviceEnumerator = unsafe { std::mem::zeroed() };

    unsafe {
        winapi::um::combaseapi::CoCreateInstance(&cls_mm_device_enum,
            std::ptr::null_mut(),
            wtypesbase::CLSCTX_INPROC_SERVER,
            &iid_imm_device_enumerator,
            &mut device_enumerator as *mut *mut IMMDeviceEnumerator
                                    as *mut *mut winapi::ctypes::c_void);
    }
    return device_enumerator;
}

fn get_imm_device(device_enumerator : *mut IMMDeviceEnumerator) -> *mut IMMDevice {
    let mut pp_device : *mut winapi::um::mmdeviceapi::IMMDevice = unsafe { std::mem::zeroed() }; 
    unsafe {
        (*device_enumerator).GetDefaultAudioEndpoint(
            winapi::um::mmdeviceapi::eCapture,
            winapi::um::mmdeviceapi::eCommunications,
            &mut pp_device
        );
    }
    return pp_device;
}

fn get_iaudio_meter_information(pp_device : *mut IMMDevice) -> *mut IAudioMeterInformation {
    let cls_iaudio_meter_information = IAudioMeterInformation::uuidof();

    let mut input_device : *mut IAudioMeterInformation = unsafe { std::mem::zeroed() };

    unsafe {
        (*pp_device).Activate(
            &cls_iaudio_meter_information,
            wtypesbase::CLSCTX_INPROC_SERVER,
            std::ptr::null_mut(),
            &mut input_device as *mut *mut winapi::um::endpointvolume::IAudioMeterInformation
                                as *mut *mut winapi::ctypes::c_void); 
    }
    return input_device;
}

pub fn get_audio_meter_information() -> *mut IAudioMeterInformation {
    unsafe { CoInitialize(std::ptr::null_mut()) };
    let device_enumerator : *mut IMMDeviceEnumerator = get_device_enumerator();
    let pp_device : *mut IMMDevice = get_imm_device(device_enumerator);
    return get_iaudio_meter_information(pp_device);
}

use std::thread;
use lazy_static::lazy_static; 
use std::sync::{Arc, Mutex};
/*
lazy_static! {
  static ref VOLUME: Mutex<f64> = Mutex::new(0.0f64);
}*/

pub fn monitor_volume(VOLUME: Mutex<f64>) -> () {
  unsafe { CoInitialize(std::ptr::null_mut()) };
  let device_enumerator : *mut IMMDeviceEnumerator = get_device_enumerator();
  let pp_device: *mut IMMDevice = get_imm_device(device_enumerator);
  let input_device: *mut IAudioMeterInformation = get_iaudio_meter_information(pp_device);

  loop {
    let mut volume: f32 = 0.0;
    unsafe { (*input_device).GetPeakValue(&mut volume) };
    //println!("{:?}", volume);
    *VOLUME.lock().unwrap() = volume as f64;
  }
}
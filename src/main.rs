#![windows_subsystem="windows"]
#[macro_use]
extern crate sciter;
extern crate winapi;
//mod windows;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref VOLUME: Mutex<f64> = Mutex::new(0.0f64);
}

static mut V: f64 = 0.5f64;

use sciter::Value;
use std::thread;
struct EventHandler;
impl EventHandler {
    fn get_volume(&self) -> f64 {
        //thread::spawn(move || {
        //monitor_volume();
        //});
        unsafe {
            return V;
        }
    }

    fn monitor_volume(&self /*, callback: Value*/) -> () {
        thread::spawn(move || {
            monitor_volume(/*callback*/);
        });

        /*thread::spawn(move || {
            loop {
                //callback.call(None, &make_args!(*VOLUME.lock().unwrap()), None).unwrap();
                unsafe { callback.call(None, &make_args!(V), None).unwrap(); }
            }
        });*/
    }
}

impl sciter::EventHandler for EventHandler {
    fn get_subscription(&mut self) -> Option<sciter::dom::event::EVENT_GROUPS> {
        Some(
            sciter::dom::event::default_events()
                | sciter::dom::event::EVENT_GROUPS::HANDLE_METHOD_CALL,
        )
    }
    dispatch_script_call!(
        fn monitor_volume(/*Value*/);
        fn get_volume();
    );
}

fn main() {
    thread::spawn(move || {
        monitor_volume();
    });

    thread::spawn(move || {
        record();
    });

    sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).unwrap();
    let archived = include_bytes!("../target/assets.rc");
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_EVAL as u8,
    ))
    .unwrap();
    let mut frame = sciter::Window::new();
    frame.event_handler(EventHandler {});
    frame.archive_handler(archived).unwrap();
    frame.load_file("this://app/html/main.html");
    frame.run_app();
}

use winapi::shared::*;
use winapi::um::endpointvolume::*;
use winapi::um::mmdeviceapi::*;
use winapi::um::objbase::*;
use winapi::Interface;

fn get_device_enumerator() -> *mut IMMDeviceEnumerator {
    let cls_mm_device_enum: guiddef::GUID = CLSID_MMDeviceEnumerator;
    let iid_imm_device_enumerator = IMMDeviceEnumerator::uuidof();

    let mut device_enumerator: *mut IMMDeviceEnumerator = unsafe { std::mem::zeroed() };

    unsafe {
        winapi::um::combaseapi::CoCreateInstance(
            &cls_mm_device_enum,
            std::ptr::null_mut(),
            wtypesbase::CLSCTX_INPROC_SERVER,
            &iid_imm_device_enumerator,
            &mut device_enumerator as *mut *mut IMMDeviceEnumerator
                as *mut *mut winapi::ctypes::c_void,
        );
    }
    return device_enumerator;
}

fn get_imm_device(device_enumerator: *mut IMMDeviceEnumerator) -> *mut IMMDevice {
    let mut pp_device: *mut winapi::um::mmdeviceapi::IMMDevice = unsafe { std::mem::zeroed() };
    unsafe {
        (*device_enumerator).GetDefaultAudioEndpoint(
            winapi::um::mmdeviceapi::eCapture,
            winapi::um::mmdeviceapi::eCommunications,
            &mut pp_device,
        );
    }
    return pp_device;
}

fn get_iaudio_meter_information(pp_device: *mut IMMDevice) -> *mut IAudioMeterInformation {
    let cls_iaudio_meter_information = IAudioMeterInformation::uuidof();

    let mut input_device: *mut IAudioMeterInformation = unsafe { std::mem::zeroed() };

    unsafe {
        (*pp_device).Activate(
            &cls_iaudio_meter_information,
            wtypesbase::CLSCTX_INPROC_SERVER,
            std::ptr::null_mut(),
            &mut input_device as *mut *mut winapi::um::endpointvolume::IAudioMeterInformation
                as *mut *mut winapi::ctypes::c_void,
        );
    }
    return input_device;
}

pub fn get_audio_meter_information() -> *mut IAudioMeterInformation {
    unsafe { CoInitialize(std::ptr::null_mut()) };
    let device_enumerator: *mut IMMDeviceEnumerator = get_device_enumerator();
    let pp_device: *mut IMMDevice = get_imm_device(device_enumerator);
    return get_iaudio_meter_information(pp_device);
}

pub fn monitor_volume(/*callback: Value*/) -> () {
    unsafe { CoInitialize(std::ptr::null_mut()) };
    let device_enumerator: *mut IMMDeviceEnumerator = get_device_enumerator();
    let pp_device: *mut IMMDevice = get_imm_device(device_enumerator);
    let input_device: *mut IAudioMeterInformation = get_iaudio_meter_information(pp_device);

    loop {
        let mut volume: f32 = 0.0;
        unsafe { (*input_device).GetPeakValue(&mut volume) };
        //*VOLUME.lock().unwrap() = volume as f64;
        //println!("rs {:?}", volume);
        unsafe {
            V = volume as f64;
        }
        /*unsafe {
            callback.call(None, &make_args!(V), None).unwrap();
        }*/
    }
}


/********************************************************/

extern crate crossbeam;
//extern crate hound;
extern crate soundio;

use std::env;
use std::fs::File;
use std::io;
use std::io::BufWriter;

//struct WavRecorder {
//    writer: hound::WavWriter<BufWriter<File>>,
//}
/*
impl WavRecorder {
    fn read_callback(&mut self, stream: &mut soundio::InStreamReader) {
        let mut frames_left = stream.frame_count_max();

        // libsoundio reads samples in chunks, so we need to loop until there's nothing to read.
        loop {
            if let Err(e) = stream.begin_read(frames_left) {
                println!("Error reading from stream: {}", e);
                return;
            }
            for f in 0..stream.frame_count() {
                for c in 0..stream.channel_count() {
                    // In reality you shouldn't write to disk in the callback, but have some buffer instead.
                    match self.writer.write_sample(stream.sample::<i16>(c, f)) {
                        Ok(_) => {}
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }

            frames_left -= stream.frame_count();
            if frames_left <= 0 {
                break;
            }

            stream.end_read();
        }
    }
}*/

// Print sound soundio debug info and record a sound.
fn record() -> Result<(), String> {
    // TODO: Probe which channels/sample rates are available.
    let channels = 2;
    let sample_rate = 44100;

    /*let spec = hound::WavSpec {
        channels: channels,
        sample_rate: sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };*/

    // Try to open the output file.
    //let writer = hound::WavWriter::create("foo.wav", spec).map_err(|x| x.to_string())?;

    //println!("Soundio version: {}", soundio::version_string());

    let mut ctx = soundio::Context::new();
    ctx.set_app_name("Recorder");
    ctx.connect()?;

    //println!("Current backend: {:?}", ctx.current_backend());

    // We have to flush events so we can scan devices.
    ctx.flush_events();
    // I guess these are always signed little endian?
    let soundio_format = soundio::Format::S16LE;

    let default_layout = soundio::ChannelLayout::get_default(channels as _);
    /*println!(
        "Default layout for {} channel(s): {:?}",
        channels, default_layout
    );*/

    let input_dev = ctx
        .default_input_device()
        .map_err(|_| "Error getting default input device".to_string())?;

    /*println!(
        "Default input device: {} {}",
        input_dev.name(),
        if input_dev.is_raw() { "raw" } else { "cooked" }
    );*/

    //let mut recorder = WavRecorder { writer: writer };

    //println!("Opening default input stream");
    let mut input_stream = input_dev.open_instream(
        sample_rate as _,
        soundio_format,
        default_layout,
        0.1,
        |x| (),
        //|x| println!("foo"),
        //|x| recorder.read_callback(x),
        None::<fn()>,
        None::<fn(soundio::Error)>,
    )?;

    //println!("Starting stream");
    input_stream.start()?;

    std::thread::sleep(std::time::Duration::from_millis(u64::MAX));

    // Wait for the user to press a key.
    //println!("Press enter to stop recording");
    //let stdin = io::stdin();
    //let input = &mut String::new();
    //let _ = stdin.read_line(input);

    Ok(())
}

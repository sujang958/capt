use std::{sync::Mutex, thread};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use xcap::{Monitor, VideoRecorder};

#[derive(Serialize, Deserialize, Debug)]
pub struct FeDisplay {
    name: String,
    width: u32,
    height: u32,
    id: u32,
    fps: f32,
}

static IS_RECORDING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static SAVE_REQUESTED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub fn get_displays() -> Vec<FeDisplay> {
    let targerts = scrap::Display::all().unwrap();

    
    // let monitors = Monitor::all().unwrap();

    // monitors
    //     .iter()
    //     .map(|monitor| FeDisplay {
    //         name: monitor.name().unwrap_or("Display".to_string()),
    //         width: monitor.width().unwrap_or(0),
    //         height: monitor.height().unwrap_or(0),
    //         id: monitor.id().unwrap_or(0),
    //         fps: monitor.frequency().unwrap_or(0f32),
    //     })
    //     .collect()
}

pub fn start_record(id: u32) {
    let binding = Monitor::all().unwrap();
    let monitor = binding.iter().find(|monitor| monitor.id().unwrap() == id);

    let (recorder, sx) = monitor.unwrap().video_recorder().unwrap();

    let rec_thread = thread::spawn(move || loop {
        

        match sx.recv() {
            Ok(frame) => {
                println!("frame: {:?}", frame.width);
            }
            _ => continue,
        }
    });
}

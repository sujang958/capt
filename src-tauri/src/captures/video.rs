use serde::{Deserialize, Serialize};
use xcap::Monitor;

#[derive(Serialize, Deserialize, Debug)]
pub struct FeDisplay {
    name: String,
    width: u32,
    height: u32,
    id: u32,
    fps: f32,
}

pub fn get_displays() -> Vec<FeDisplay> {
    let monitors = Monitor::all().unwrap();

    monitors
        .iter()
        .map(|monitor| FeDisplay {
            name: monitor.name().unwrap_or("Display".to_string()),
            width: monitor.width().unwrap_or(0),
            height: monitor.height().unwrap_or(0),
            id: monitor.id().unwrap_or(0),
            fps: monitor.frequency().unwrap_or(0f32),
        })
        .collect()
}

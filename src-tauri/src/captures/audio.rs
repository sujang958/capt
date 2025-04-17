// use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// use std::collections::VecDeque;
// use std::sync::{Arc, Mutex};

// const SAMPLE_RATE: usize = 48000; // usually 44100 or 48000
// const CHANNELS: usize = 1; // mono
// const SECONDS: usize = 30;
// const MAX_SAMPLES: usize = SAMPLE_RATE * CHANNELS * SECONDS;

// fn capture_audio() -> Result<(), anyhow::Error> {
//     let host = cpal::default_host();
//     let device = host.default_input_device().expect("No input device");
//     let config = device.default_input_config()?.into();

//     let buffer: Arc<Mutex<VecDeque<f32>>> = Arc::new(Mutex::new(VecDeque::with_capacity(MAX_SAMPLES)));
//     let buffer_clone = buffer.clone();

//     let stream = device.build_input_stream(
//         &config,
//         move |data: &[f32], _| {
//             let mut buf = buffer_clone.lock().unwrap();
//             for &sample in data {
//                 if buf.len() >= MAX_SAMPLES {
//                     buf.pop_front(); // remove oldest
//                 }
//                 buf.push_back(sample);
//             }
//         },
//         move |err| {
//             eprintln!("Stream error: {:?}", err);
//         },
//         None,
//     )?;

//     stream.play()?;
//     println!("Recording... keeping last 30 seconds.");

//     // Recording loop (or could be triggered by UI)
//     std::thread::sleep(std::time::Duration::from_secs(40)); // just demo runtime

//     // Capture snapshot
//     let snapshot: Vec<f32> = buffer.lock().unwrap().clone().into();

//     println!("Captured {} samples.", snapshot.len());

//     // You can now write this to a file, stream, or encode.
//     Ok(())
// }

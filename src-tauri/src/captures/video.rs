use std::collections::VecDeque;
use std::{thread, time::Duration};
use scrap::{Capturer, Display};
use image::{ImageBuffer, Rgba};
use std::sync::{Arc, Mutex};

const FPS: usize = 10;
const SECONDS: usize = 30;
const MAX_FRAMES: usize = FPS * SECONDS;

fn capture_video() -> Result<(), Box<dyn std::error::Error>> {
    let display = Display::primary()?;
    let mut capturer = Capturer::new(display)?;
    let width = capturer.width();
    let height = capturer.height();

    let buffer: Arc<Mutex<VecDeque<ImageBuffer<Rgba<u8>, Vec<u8>>>>> =
        Arc::new(Mutex::new(VecDeque::with_capacity(MAX_FRAMES)));
    let buffer_clone = buffer.clone();

    std::thread::spawn(move || loop {
        if let Ok(frame) = capturer.frame() {
            let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width as u32, height as u32);
            for (i, pixel) in frame.chunks(4).enumerate() {
                let x = (i % width) as u32;
                let y = (i / width) as u32;
                img.put_pixel(x, y, Rgba([pixel[2], pixel[1], pixel[0], 255]));
            }

            let mut buf = buffer_clone.lock().unwrap();
            if buf.len() >= MAX_FRAMES {
                buf.pop_front();
            }
            buf.push_back(img);
        }

        thread::sleep(Duration::from_millis(1000 / FPS as u64));
    });

    // Simulate waiting
    thread::sleep(Duration::from_secs(40));

    // Snapshot
    let frames = buffer.lock().unwrap().clone();
    println!("Captured {} frames (last 30 sec)", frames.len());

    Ok(())
}

use std::io::ErrorKind::WouldBlock;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::thread;
use std::time::Duration;
use image;
use scrap::{self, Display, Capturer};

fn main() {
    list_displays();
    take_screenshot();
}

fn list_displays() {
    println!("Listing displays:");
    let displays = Display::all().unwrap();
    for (i, display) in displays.iter().enumerate() {
        println!("    Display {} [{}x{}]",
                 i + 1,
                 display.width(),
                 display.height());
    }
}

fn take_screenshot() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;
    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());
    loop {
        // Wait until there's a frame.
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };
        println!("Captured! Saving...");
        // Flip the ARGB image into a BGRA image.
        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;
        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                    255,
                ]);
            }
        }
        // Save the image.
        image::save_buffer("s.png", &buffer, w as u32, h as u32, image::ColorType::Rgba8);
        // image::save_buffer("s.png", &buffer, w as u32, h as u32, image::ColorType::Bgra8);
        println!("Image saved to s.png.");
        dump_buffer(&buffer, h, w);
        break;
    }
}

fn dump_buffer(buffer: &[u8], h: usize, w: usize) {
    /* TODO: confused with about BufWriter.write ...
    bfw.write(&buffer);
    */
    let f = File::create("buf.txt").expect("Unable to create file");
    let mut bfw = BufWriter::new(f);
    let stride = buffer.len() / h;
    for y in 0..h {
        for x in 0..w {
            let i = stride * y + 4 * x;
            bfw.write(buffer[i].to_string().as_bytes());
            bfw.write(b":");
            bfw.write(buffer[i + 1].to_string().as_bytes());
            bfw.write(b":");
            bfw.write(buffer[i + 2].to_string().as_bytes());
            bfw.write(b":");
            bfw.write(buffer[i + 3].to_string().as_bytes());
            bfw.write(b" ");
        }
        bfw.write(b"\n");
    }
}

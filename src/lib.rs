extern crate rgb;
extern crate serial;
extern crate serial_unix;

use std::io::Write;

const BLACK: rgb::RGB8 = rgb::RGB8 { r: 0, g: 0, b: 0 };

pub const PIXELS_PER_CHANNEL: u8 = 16;
pub const CHANNELS: u8 = 4;
pub const TOTAL_PIXELS: usize = PIXELS_PER_CHANNEL as usize * CHANNELS as usize;

pub struct Mote {
    port: serial_unix::TTYPort,
    current: [rgb::RGB8; TOTAL_PIXELS as usize],
}

impl Mote {
    pub fn new(path: &str, gamma_correction: bool) -> Mote {
        let mut mote = Mote {
            port: serial_unix::TTYPort::open(std::path::Path::new(path)).unwrap(),
            current: [BLACK; TOTAL_PIXELS],
        };
        mote.init(gamma_correction);
        mote.clear();
        mote
    }

    pub fn init(&mut self, gamma_correction: bool) {
        for channel in 0..CHANNELS {
            // Channels are one-based.
            self.configure_channel(channel + 1, PIXELS_PER_CHANNEL, gamma_correction);
        }
    }

    pub fn clear(&mut self) {
        self.write(&[BLACK; TOTAL_PIXELS])
    }

    fn configure_channel(&mut self, channel: u8, num_pixels: u8, gamma_correction: bool) {
        self.port.write_all(b"mote").unwrap();
        self.port.write_all(b"c").unwrap();
        self.port.write_all(&[channel]).unwrap();
        self.port.write_all(&[num_pixels]).unwrap();
        self.port
            .write_all(&[if gamma_correction { 1 } else { 0 }])
            .unwrap();
    }

    pub fn read(&self) -> &[rgb::RGB8; TOTAL_PIXELS] {
        &self.current
    }

    pub fn write(&mut self, pixels: &[rgb::RGB8; TOTAL_PIXELS]) {
        self.port.write_all(b"mote").unwrap();
        self.port.write_all(b"o").unwrap();
        for pixel in pixels.iter() {
            self.port.write_all(&[pixel.b, pixel.g, pixel.r]).unwrap();
        }
        self.current = *pixels;
    }
}

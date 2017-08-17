extern crate serial_unix;
extern crate serial;
extern crate rgb;

use std::io::Write;

const BLACK: rgb::RGB8 = rgb::RGB8 { r: 0, g: 0, b: 0 };

pub struct Mote {
    port: serial_unix::TTYPort,
}

impl Mote {
    pub fn new(path: &str, gamma_correction: bool) -> Mote {
        let mut mote = Mote {
            port: serial_unix::TTYPort::open(std::path::Path::new(path)).unwrap(),
        };
        mote.init(gamma_correction);
        mote.clear();
        mote
    }

    pub fn init(&mut self, gamma_correction: bool) {
        self.configure_channel(1, 16, gamma_correction);
        self.configure_channel(2, 16, gamma_correction);
        self.configure_channel(3, 16, gamma_correction);
        self.configure_channel(4, 16, gamma_correction);
    }

    pub fn clear(&mut self) {
        self.write(&[BLACK; 16 * 4])
    }

    fn configure_channel(&mut self, channel: u8, num_pixels: u8, gamma_correction: bool) {
        // 'mote'
        self.port.write(&[0x6D, 0x6F, 0x74, 0x65]).unwrap();
        // 'c'
        self.port.write(&[0x63]).unwrap();
        self.port.write(&[channel]).unwrap();
        self.port.write(&[num_pixels]).unwrap();
        self.port
            .write(&[if gamma_correction { 1 } else { 0 }])
            .unwrap();
    }

    pub fn write(&mut self, pixels: &[rgb::RGB8]) {
        // 'mote'
        self.port.write(&[0x6D, 0x6F, 0x74, 0x65]).unwrap();
        // 'o'
        self.port.write(&[0x6F]).unwrap();
        for pixel in pixels {
            self.port.write(&[pixel.b, pixel.g, pixel.r]).unwrap();
        }
    }
}

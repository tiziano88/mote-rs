extern crate serial_unix;
extern crate serial;
extern crate rgb;

use std::io::Write;

const BLACK: rgb::RGB8 = rgb::RGB8 { r: 0, g: 0, b: 0 };

const RED: rgb::RGB8 = rgb::RGB8 {
    r: 255,
    g: 0,
    b: 0,
};

const GREEN: rgb::RGB8 = rgb::RGB8 {
    r: 0,
    g: 255,
    b: 0,
};

const BLUE: rgb::RGB8 = rgb::RGB8 {
    r: 0,
    g: 0,
    b: 255,
};

struct Mote {
    port: serial_unix::TTYPort,
}

impl Mote {
    pub fn new(path: &str) -> Mote {
        let mut mote =
            Mote { port: serial_unix::TTYPort::open(std::path::Path::new(path)).unwrap() };
        mote.init();
        mote.clear();
        mote
    }

    pub fn init(&mut self) {
        self.configure_channel(1, 16, false);
        self.configure_channel(2, 16, false);
        self.configure_channel(3, 16, false);
        self.configure_channel(4, 16, false);
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
        self.port.write(&[if gamma_correction { 1 } else { 0 }]).unwrap();
    }

    fn write(&mut self, pixels: &[rgb::RGB8]) {
        // 'mote'
        self.port.write(&[0x6D, 0x6F, 0x74, 0x65]).unwrap();
        // 'o'
        self.port.write(&[0x6F]).unwrap();
        for pixel in pixels {
            self.port.write(&[pixel.b, pixel.g, pixel.r]).unwrap();
        }
    }
}

#[test]
fn it_works() {
    let mut mote = Mote::new("/dev/ttyACM0");
    mote.clear();
    mote.write(&[RED; 16 * 4]);
}

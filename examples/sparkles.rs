extern crate getopts;
extern crate mote;
extern crate rand;
extern crate rgb;

use getopts::Options;
use rand::distributions::IndependentSample;
use std::env;

const BACKGROUND: rgb::RGB8 = rgb::RGB8 { r: 0, g: 20, b: 0 };

const FOREGROUND: rgb::RGB8 = rgb::RGB8 { r: 0, g: 150, b: 0 };

fn main() {
    let mut opts = Options::new();
    opts.optopt("", "device", "device path", "FILE");
    let matches = opts.parse(env::args()).unwrap();

    let path = matches
        .opt_str("device")
        .unwrap_or("/dev/ttyACM0".to_string());

    let mut mote = mote::Mote::new(&path, true);
    mote.clear();

    let between = rand::distributions::Range::new(0, mote::TOTAL_PIXELS);
    let mut rng = rand::thread_rng();

    println!("start");
    let base = [BACKGROUND; mote::TOTAL_PIXELS];
    mote.write(&base);
    let mut current = base;

    let mut n = 0;
    loop {
        if n % 2 == 0 {
            n = 0;
            let i = between.ind_sample(&mut rng);
            current[i] = FOREGROUND;
        }
        for i in 0..mote::TOTAL_PIXELS {
            current[i] = mean(current[i], base[i], 0.92);
        }
        mote.write(&current);
        std::thread::sleep(std::time::Duration::from_millis(1));
        n += 1;
    }
}

fn mean(x: rgb::RGB8, y: rgb::RGB8, p: f32) -> rgb::RGB8 {
    rgb::RGB8 {
        r: (x.r as f32 * p + y.r as f32 * (1.0 - p)) as u8,
        g: (x.g as f32 * p + y.g as f32 * (1.0 - p)) as u8,
        b: (x.b as f32 * p + y.b as f32 * (1.0 - p)) as u8,
    }
}

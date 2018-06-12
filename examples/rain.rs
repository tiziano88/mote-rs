extern crate getopts;
extern crate mote;
extern crate rand;
extern crate rgb;

use getopts::Options;
use rand::distributions::{Distribution, IndependentSample};
use std::env;

const BACKGROUND: rgb::RGB8 = rgb::RGB8 { r: 0, g: 0, b: 0 };

struct Particle {
    creation_time: u64,
    color: rgb::RGB8,
}

fn main() {
    let mut opts = Options::new();
    opts.optopt("", "device", "device path", "FILE");
    let matches = opts.parse(env::args()).unwrap();

    let path = matches
        .opt_str("device")
        .unwrap_or("/dev/ttyACM0".to_string());

    let mut mote = mote::Mote::new(&path, true);
    mote.clear();

    let dist = rand::distributions::Poisson::new(0.4);
    let mut rng = rand::thread_rng();

    println!("start");
    let mut current = [BACKGROUND; mote::TOTAL_PIXELS];
    mote.write(&current);

    // Speed in pixels per cycle.
    const SPEED: f32 = 0.5;

    let mut particles = Vec::<Particle>::new();

    let mut n = 0u64;
    loop {
        if dist.sample(&mut rng) > 1 {
            particles.push(Particle {
                creation_time: n,
                color: random_color(),
            });
        }

        let mut mask = [BACKGROUND; mote::TOTAL_PIXELS];
        for p in particles.iter() {
            let x = ((n - p.creation_time) as f32 * SPEED) as usize;
            if x < mask.len() {
                mask[x] = p.color;
            }
        }

        for i in 0..mote::TOTAL_PIXELS {
            current[i] = add(current[i], mask[i]);
            current[i] = mean(current[i], BACKGROUND, 0.88);
        }

        mote.write(&current);
        std::thread::sleep(std::time::Duration::from_millis(100));
        n += 1;
    }
}

fn random_color() -> rgb::RGB8 {
    let between = rand::distributions::Range::<u8>::new(0, 255);
    let mut rng = rand::thread_rng();
    let r = between.ind_sample(&mut rng);
    let g = between.ind_sample(&mut rng);
    let b = between.ind_sample(&mut rng);
    rgb::RGB8 { r, g, b }
}

fn mean(x: rgb::RGB8, y: rgb::RGB8, p: f32) -> rgb::RGB8 {
    rgb::RGB8 {
        r: (x.r as f32 * p + y.r as f32 * (1.0 - p)) as u8,
        g: (x.g as f32 * p + y.g as f32 * (1.0 - p)) as u8,
        b: (x.b as f32 * p + y.b as f32 * (1.0 - p)) as u8,
    }
}

fn screen(x: rgb::RGB8, y: rgb::RGB8) -> rgb::RGB8 {
    rgb::RGB8 {
        r: 255 - (255 - x.r) * (255 - y.r),
        g: 255 - (255 - x.g) * (255 - y.g),
        b: 255 - (255 - x.b) * (255 - y.b),
    }
}

fn add(x: rgb::RGB8, y: rgb::RGB8) -> rgb::RGB8 {
    rgb::RGB8 {
        r: std::cmp::min(x.r as u16 + y.r as u16, 255) as u8,
        g: std::cmp::min(x.g as u16 + y.g as u16, 255) as u8,
        b: std::cmp::min(x.b as u16 + y.b as u16, 255) as u8,
    }
}

extern crate getopts;
extern crate mote;
extern crate palette;
extern crate rand;
extern crate rand_distr;
extern crate rgb;

use getopts::Options;
use palette::blend::Blend;
use palette::Mix;
use rand_distr::Distribution;
use std::env;

const BACKGROUND: palette::rgb::LinSrgb = palette::rgb::LinSrgb {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    standard: std::marker::PhantomData,
};

struct Particle {
    creation_time: u64,
    color: palette::rgb::LinSrgb,
}

struct Segment {
    particles: Vec<Particle>,
    pixels: Vec<palette::rgb::LinSrgb>,
}

impl Default for Segment {
    fn default() -> Self {
        Segment {
            particles: vec![],
            pixels: [BACKGROUND; mote::PIXELS_PER_CHANNEL as usize].to_vec(),
        }
    }
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

    let dist = rand_distr::Poisson::new(0.5f32).unwrap();
    let mut rng = rand::thread_rng();

    let mut segments = vec![
        Segment::default(),
        Segment::default(),
        Segment::default(),
        Segment::default(),
    ];

    println!("start");
    {
        let blank = [BACKGROUND; mote::TOTAL_PIXELS];
        mote.write(&to_array(&blank.iter().map(to_rgb).collect::<Vec<_>>()));
    }

    let mut n = 0u64;
    loop {
        for segment in segments.iter_mut() {
            let v: f32 = dist.sample(&mut rng);
            if v > 1.0 {
                segment.particles.push(Particle {
                    creation_time: n,
                    color: random_color(),
                });
            }
            let mask = make_mask(&segment.particles, n);
            for i in 0..mote::PIXELS_PER_CHANNEL as usize {
                segment.pixels[i] = segment.pixels[i].screen(mask[i]);
                segment.pixels[i] = segment.pixels[i].mix(&BACKGROUND, 0.20);
            }
        }

        let pixels = segments
            .iter()
            .flat_map(|x| x.pixels.clone())
            .collect::<Vec<_>>();
        mote.write(&to_array(&pixels.iter().map(to_rgb).collect::<Vec<_>>()));

        std::thread::sleep(std::time::Duration::from_millis(50));
        n += 1;
    }
}

fn to_array(pixels: &[rgb::RGB8]) -> [rgb::RGB8; mote::TOTAL_PIXELS] {
    let mut out = [to_rgb(&BACKGROUND); mote::TOTAL_PIXELS];
    out.clone_from_slice(pixels);
    out
}

fn make_mask(particles: &[Particle], n: u64) -> [palette::rgb::LinSrgb; mote::TOTAL_PIXELS] {
    // Speed in pixels per cycle.
    const SPEED: f32 = 0.5;
    let mut mask = [BACKGROUND; mote::TOTAL_PIXELS];
    for p in particles.iter() {
        let x = ((n - p.creation_time) as f32 * SPEED) as usize;
        if x < mask.len() {
            mask[x] = p.color;
        }
    }
    mask
}

fn random_color() -> palette::rgb::LinSrgb {
    let between = rand::distributions::Uniform::new(0f32, 360f32);
    let mut rng = rand::thread_rng();
    let h = palette::RgbHue::<f32>::from_degrees(between.sample(&mut rng));
    let s = 1.0;
    let v = 0.5;
    palette::Hsv::new(h, s, v).into()
}

fn to_rgb(c: &palette::rgb::LinSrgb) -> rgb::RGB8 {
    rgb::RGB8 {
        r: (c.red * 255.0) as u8,
        g: (c.green * 255.0) as u8,
        b: (c.blue * 255.0) as u8,
    }
}

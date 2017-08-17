extern crate mote;
extern crate rand;
extern crate rgb;

use rand::distributions::IndependentSample;

const RED: rgb::RGB8 = rgb::RGB8 { r: 255, g: 0, b: 0 };

const GREEN: rgb::RGB8 = rgb::RGB8 { r: 0, g: 255, b: 0 };

const BLUE: rgb::RGB8 = rgb::RGB8 { r: 0, g: 0, b: 255 };

const YELLOW: rgb::RGB8 = rgb::RGB8 {
    r: 255,
    g: 255,
    b: 0,
};


fn main() {
    let mut mote = mote::Mote::new("/dev/ttyACM0", true);
    mote.clear();

    let between = rand::distributions::Range::new(0, 16 * 4);
    let mut rng = rand::thread_rng();

    println!("start");
    let base = [RED; 16 * 4];
    mote.write(&base);
    let mut current = base;

    let mut n = 0;
    loop {
        if n % 2 == 0 {
            n = 0;
            let i = between.ind_sample(&mut rng);
            current[i] = YELLOW;
        }
        for i in 0..16 * 4 {
            current[i] = mean(current[i], base[i], 0.85);
        }
        mote.write(&current);
        std::thread::sleep(std::time::Duration::from_millis(10));
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

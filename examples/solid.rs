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

    let between = rand::distributions::Range::new(0, mote::TOTAL_PIXELS);
    let mut rng = rand::thread_rng();

    println!("start");
    let base = [RED; mote::TOTAL_PIXELS];
    mote.write(&base);
}

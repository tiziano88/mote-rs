extern crate mote;
extern crate rgb;

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

fn main() {
    let mut mote = mote::Mote::new("/dev/ttyACM0");
    mote.clear();
    mote.write(&[GREEN; 16 * 4]);
}

extern crate mote;
extern crate rgb;

const WHITE: rgb::RGB8 = rgb::RGB8 {
    r: 255,
    g: 255,
    b: 255,
};

const BLACK: rgb::RGB8 = rgb::RGB8 { r: 0, g: 0, b: 0 };

fn main() {
    let mut mote = mote::Mote::new("/dev/ttyACM0", true);
    mote.clear();

    println!("start");

    let mut n = 0;
    loop {
        if n % 2 == 0 {
            n = 0;
            mote.write(&[WHITE; 16 * 4]);
        } else {
            mote.write(&[BLACK; 16 * 4]);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        n += 1;
    }
}

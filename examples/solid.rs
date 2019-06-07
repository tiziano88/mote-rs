extern crate mote;
extern crate rgb;

const RED: rgb::RGB8 = rgb::RGB8 { r: 255, g: 0, b: 0 };

const GREEN: rgb::RGB8 = rgb::RGB8 { r: 0, g: 255, b: 0 };

const BLUE: rgb::RGB8 = rgb::RGB8 { r: 0, g: 0, b: 255 };

const YELLOW: rgb::RGB8 = rgb::RGB8 {
    r: 255,
    g: 255,
    b: 0,
};

fn main() {
    let c = parse_color().unwrap_or(RED);
    println!("color: {:?}", c);

    let mut mote = mote::Mote::new("/dev/ttyACM0", true);
    mote.clear();

    println!("start");
    let base = [c; mote::TOTAL_PIXELS];
    mote.write(&base);
}

fn parse_color() -> Option<rgb::RGB8> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 4 {
        let r = args[1].parse::<u8>().ok()?;
        let g = args[2].parse::<u8>().ok()?;
        let b = args[3].parse::<u8>().ok()?;
        Option::Some(rgb::RGB8 { r, g, b })
    } else {
        Option::None
    }
}

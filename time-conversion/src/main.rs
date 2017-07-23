extern crate regex;
use regex::Regex;
use std::io;

fn main() {
    let re = Regex::new(r"^([01][0-9]):([0-5][0-9]):([0-5][0-9])([AP])M$")
                        .expect(format!("{0}:{1}", file!(), line!()).as_str());
    let mut line0 = String::new();

    io::stdin().read_line(&mut line0).unwrap();

    let caps = re.captures(line0.trim()).expect(format!("{0}:{1}", file!(), line!()).as_str());
    let hours_num: u8 = caps[1].parse().expect(format!("-{0}:{1}", file!(), line!()).as_str());

    let hours = match &caps[4] {
        "P" => match hours_num {
            12 => 12,
            _ => 12u8 + hours_num,
        },
        "A" => match hours_num {
            12 => 0,
            _ => hours_num,
        },
        _ => panic!("Unsupported universe!"),
    };

    print!("{0:02}:{1}:{2}", hours, &caps[2], &caps[3]);
}

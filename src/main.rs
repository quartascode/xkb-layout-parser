use std::io::Write;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path;

fn main() {
    let file = File::open("/home/breno/.config/xkb/symbols/xkb-test").unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        print!("{}\n", line);
    }
}

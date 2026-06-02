use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("/home/breno/.config/xkb/symbols/mc");
    let path_display = path.display();

    let mut layout_file = match File::open(&path) {
        Ok(layout_file) => layout_file,
        Err(why) => panic!("Couldn't open {}: {}", path_display, why),
    };

    let mut s = String::new();
    match layout_file.read_to_string(&mut s) {
        Err(why) => panic!("Coulnd't read {}: {}", path_display, why),
        Ok(_) => {},
    };

    let mut output = String::new();
    for c in 0..s.len() {
        // ajeita essa bosta
        if s.chars()[c] == 'k' {
            output.push(c);
        }
    }

    println!("{}", output);
    println!("{:?}", s.chars());
}

use std::{fs, os};
fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    println!("{:?}", args);

    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
 
    let mut _array = vec!["123","hello"];
    _array.concat([b"world"]);
    println!("{:?}", _array);
    0
}

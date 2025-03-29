use std::fs;

pub fn print_dir() -> std::io::Result<()> {
    for entry in fs::read_dir("./")? {
        // let dir = entry?;
        println!("{:?}", entry?.path());
    }
    Ok(())
}

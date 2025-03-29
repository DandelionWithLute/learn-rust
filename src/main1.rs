mod print_dir;
mod write_sample;
mod zip_dir;

fn test() {
    let _r = print_dir::print_dir();

    // let _w = zip_dir::write_dir();

    // Fails
    // let _z = zip_dir::zip_dir(
    //     "test.txt",
    //     "./",
    //     zip::CompressionMethod::Aes,
    //     zip::CompressionMethod::Aes,
    // );

    let _sample = write_sample::Write("");
}

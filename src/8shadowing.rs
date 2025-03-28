fn shadowing() {
    let a = 32;
    let a = a * 3;
    println!("{}", a);
}

fn function() {
    let old = 12;
    let young = 14;
    let total = add(old, young);
    println!("All persons in total are: {}", total);
    println!("All persons in total are: {}", add(old, young));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn utf_8str() {
    let s1 = String::from("1");
    let s2 = String::from("2");

    let s3 = s1 + &s2;
    println!("{s3}");
    println!("{s1}");
    println!("{s2}");

    // let s3 = s1.clone() + &s2;
}

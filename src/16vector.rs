fn vector() {
    let _v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &_v[2];

    println!("The third element is {third}");

    let fifth_from_get = _v.get(4);

    // println!("The fifth element is {:?}", fifth_from_get);
    // Some(5)

    match fifth_from_get {
        Some(fifth_from_get) => println!("The fifth element is {:?}", fifth_from_get),
        None => println!("None"),
    }
}

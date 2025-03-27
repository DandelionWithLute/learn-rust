fn ownership() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("The length of string one is {}", len);
    let s2 = s1;
    // println!("s1:{} was borrowed by s2:{}", s1, s2)
    //                                         ^^ value borrowed here after move
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// cannot find value `s1` in this scope
// fn out_of_scope(&a: String) -> String {
//     println!("{}", s1);
// }

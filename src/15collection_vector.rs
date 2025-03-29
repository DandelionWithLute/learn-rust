// https://doc.rust-lang.org/std/vec/struct.Vec.html
// The vec! macro is provided for convenient initialization
// Slicing: A Vec can be mutable.

fn collection_vector() {
    let _vector: Vec<i32> = Vec::new();
    let mut _the_vec: Vec<i32> = vec![1, 2, 3];

    _the_vec.push(4);
    _the_vec.push(5);

    let the_new_vec = _the_vec.clone();
    // copy(_the_vec, the_new_vec);

    for item in _the_vec {
        println!("{}", item)
    }

    println!("{:?}", the_new_vec);
}

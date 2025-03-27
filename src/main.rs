fn main() {
    // 1 - Debuggable format
    // 2 - Display format
    //  {:?}
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array:{:?}", numbers);

    // let mix: [i32; 4] = [1, 2, "apple", true];
    // println!(mix)

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array:{:?}", fruits);
    println!("Fruits:{:?}", fruits[0]);
    println!("Fruits:{:?}", fruits[1]);
    println!("Fruits:{:?}", fruits[2]);
    println!("Fruits:{}", fruits[0]);
    println!("Fruits:{}", fruits[1]);
    println!("Fruits:{}", fruits[2]);

    // Tuples
    // let human: (String) = ("Alice", 30, false);
    // let human: (&str, i32, bool) = ("Alice", 30, false);
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple :{:?}", human);

    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Clod Says: {}", stone_cold);

    // slice
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice)
}

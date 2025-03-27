// https://www.youtube.com/watch?v=rQ_J9WH6CGk

// fn main() {
//     println!("Hello, world!");
// }

fn primitive() {
    // i is to the power of 32 (signed integer)
    // u is to the power of 64 (unsigned integer)
    let x: i32 = -42;
    let y: u64 = 100;

    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    // "" contains var, '' contains pure literal letters
    // let letter: char = "a";
    let letter: char = 'a';
    println!("First letter of the alphabet is {}", &letter)
}

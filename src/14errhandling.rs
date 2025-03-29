use core::f64;

// Match function or Match result of function !

fn errhandling() {
    // Approach
    // enum Option<T> {
    //     Some(T),
    //     None, // Abscence of value
    // }
    // enum Result<T, E> {
    //     OK(T),
    //     Err(E),
    // }

    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    // 2.0 or 0.0
    let res = divide(10.0, 2.0);
    match res {
        Some(x) => println!("Result:{}", x),
        None => println!("Can't devide by zero!"),
    }
}

fn extra14() {
    // enum <O,E> {
    //     Ok(O),
    //     Err(E)
    // }

    fn divide(e: f64, d: f64) -> Result<f64, String> {
        if d == 0.0 {
            Err("Can't divide by 0".to_string())
        } else {
            Ok(e / d)
        }
    }

    let res = divide(50.0, 2.3);
    match res {
        Err(e) => println!("{}", e),
        Ok(t) => println!("{}", t),
    }

    match divide(25.0, 5.0) {
        Ok(res) => println!("Result: {}", res),
        Err(err) => println!("Error: {}", err),
    }
}

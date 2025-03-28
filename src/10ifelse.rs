fn ifelse() {
    let mut counter = 0;
    let res = loop {
        counter += 1;
        // it's ok
        // counter == 20 && break counter * 2;

        if counter == 20 {
            break counter * 3;
        }
    };

    // [me] loop comes with label breaks with label !
    'another_counting_up: loop {
        counter += 2;
        println!("{}", counter);
        // break String::from("123");
        break 'another_counting_up;
    }

    println!("{}", res);

    
}

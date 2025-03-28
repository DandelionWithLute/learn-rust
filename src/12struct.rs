fn struct_function() {
    // tuple [t/\pl]
    let _rect = (200, 500);

    // struct
    struct Book {
        title: String,
    }

    let mut book: Book = Book {
        title: String::from("Pride and Prejudice"),
    };

    println!("I have a book called {}", book.title);

    book.title = String::from("Mockingbird");

    println!("I have another book called {}", book.title);
}

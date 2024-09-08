fn greet(name_param: String) -> String {
    println!("Hi {name_param}");
    name_param
}

pub fn rust_borrowing() {
    // borrowing is a technique to access the value out of its owner's scope.
    let name = String::from("Abhishek");

    // this is also a valid approach to achive the behaviour using return statement in greet function.
    let returned_name = greet(name);

    println!("{returned_name}");
    // However, this style of program is quite verbose. Rust provides a concise style of reading and writing without moves through references.

    // we can use borrowing feature to access the value through name variable.
    // first, references are a kind of pointers it means they are just pointing to a value stored in heap.
    // so we can make another variable that points to these references rather than pointing to the value in heap.
    // this technique does not takes the ownership from the initial owner.

    // for example
    let new_name: String = String::from("Lucifer");

    fn new_greet(new_name_param: &String) {
        // see how the parameter type changed to &String means a reference to a String value's reference.
        println!("Hello mr. {new_name_param}");
    }

    new_greet(&new_name); // see we do not passing the value of new_name. instead we are passing the pointer of new_name; this technique in rust is called borrowing.

    // the new_name variable still has ownership of the value "Lucifer" stored in the heap;
    println!("{new_name}");

    // an interesting fact to know is when new_greet execution finished, no deallocation of heap initiated by rust. the reason is the parameter
    // new_name_param never owned the value of heap. And deallocation happens if and only if the owner of the value stored in the heap goes out of the scope.

    // IMP: References are non-owning pointers, because they do not own the data they point to.

    
}

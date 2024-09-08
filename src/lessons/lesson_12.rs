fn greet(name_param: String) {
    println!("Hi, {name_param}");
}
fn greet_literal(name_param: &str) {
    println!("Hi, {name_param}");
}
pub fn rust_ownership_and_memory() {
    // rust uses its own way to use memory
    // it can assign data in heap and can free automatically the memory
    // you can read more at https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html#ownership-as-a-discipline-for-memory-safety

    // there are some rules/desciplines of ownership in rust.
    /*
        1. All heap data must be owned by one and only one variable at a time.
        2. Rust deallocates heap data once its owner goes out of scope.
        3. Ownership can be transferred by moves, which happen on assignments and function calls.
        4. Heap data can only be accessed through its current owner, not a previous owner.
    */

    let name = String::from("Abhishek"); // stored the value "Abhishek" in heap...
    greet(name); // passed the ownership of value "Abhishek" to the function greet or you can say more specifically the parameter "name_param".

    // since the value has been moved from name to name_param now name does not have the reference of heap location of "Abhishek";
    // this is called moves
    // println!("{name}"); // this will throw the error

    // let's see whether it is true only for heap data or any kind of data.
    {
        let surname = "Prajapati"; // here the value "Prajapati" is a string literal value.
        greet_literal(surname);
        println!("{}", surname); // Okay so that is only tru for data stored in heap. Because the value is accessible through the previous owner surname.

        // now I am wondering if the concept of scope chain applies here or not.
        {
            println!("{}", surname); // Alright it applies here.
            {
                println!("{}", surname); // Alright it applies here also and I think this can go any level deeper.
            }
        }
    }

    // At this point, I am not sure but probably the surname variable's value is getting deallcated from memory by rust itself.

    // but here we cannot access the value because the owner surname is not in this scope.
    // println!("{}", surname);

    // All the types mentioned so far are literals except the String which is significantly more complex type. And these types are stored in stack rather than heap.
    
}

fn greet(name_param: String) {
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
}

pub fn data_types_boolean() {
    // boolean values are true and false. that's it.
    let is_max = false; // inferred type boolean or implicity type

    let is_min: bool = true; // with explicit type annotation

    // a boolean value takes only one byte in the memory.

    // booleans are mainly used to decide the flow of a program
    // often used in condition statements or loops etc...
    println!("{} and {}", is_max, is_min);
}

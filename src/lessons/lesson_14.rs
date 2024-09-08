pub fn rust_borrowing_deep() {
    // Box and String methods can be used to store data in the heap. at least that is what we have learned so far.
    // But we still do not know how rust follows a pointer to its stored data.

    // I mean how println macro knew that there is difference between String & &String... ?

    // rust: that is with the help of dereference operator.
    // what the heck is that ?
    // rust: let's see...

    let mut x: Box<i32> = Box::new(23); // stored 23 in the heap...
    let a: i32 = *x; // * is the dereference operator and *x reads the value of x in the heap so a becomes 23;
    *x += 1; // we just modified the heap value so now x is pointing to the 23+1 = 24
    println!("{a}"); // a is still 23
    println!("{x}"); // but the x is now pointing to the 24

    let r1: &Box<i32> = &x; // r1 points to x on stack
    let _b: i32 = **r1; // two dereference operator get use the heap value again how ? because first operator went for r's exact value that is x and second went for the x's value.

    let r2: &i32 = &*x; // here we directly accessed heap value. how ? because & operator returned the pointer of x and * operator returned the value that pointer is pointing in the heap.
    let c: i32 = *r2; // here c will store the heap value because r2 is the pointer that references the heap value;
    println!("{c}");

    // understand that a, _b, and c are values not pointers
    // others are just pointers...

    // now we have seen the dereference operator in action

    // there are two type of dereferencing in rust 1. implicit and 2. explicit

    let num: Box<i32> = Box::new(-1);
    let abs_val = num.abs(); // dereferencing happened... implicitly
    let abs_val2 = i32::abs(*num); // dereferencing happenned... explicitly
    assert_eq!(abs_val, abs_val2);

    let num_stack: &Box<i32> = &num;
    let num_stack_abs_val: i32 = num_stack.abs(); // dereferencing happened... twice implicitly
    let num_stack_abs_val2: i32 = i32::abs(**num_stack); // dereferencing happened... twice explicitly
    assert_eq!(num_stack_abs_val, num_stack_abs_val2);

    let name: String = String::from("Abhishek");
    let name_len = name.len(); // implicit reference
    let name_len2 = str::len(&name); // explicit reference
    assert_eq!(name_len, name_len2);

    // so rust is smart enough to reference the heap value at any level deep.
    // we can see that name was of type String but len expects the value to be of type &str means there is another
    // magic happening there that is typecasting....
}

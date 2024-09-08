pub fn ownership_revisited() {
    // Now that we have idea about move and copy. We can revisit the ownership examples
    // for why they behave the way they behave.

    let s = String::from("Hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function,
                        // and so s is not longer valid afterwards.

    //println!("{}", s); // this would give an error;

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy ( a special Trait), so it's okay
                   // to still use x afterward

    /*
        If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error. These
        static checks protect us from mistakes. However, if we want to use s after the call we can return the value;
        returning the same value somehow returns the ownership also.
    */

    let mut s2 = String::from("Hello, Abhishek");
    s2 = returns_ownership(s2);

    // now s2 has the ownership back of the value "Hello, Abhishek";
    println!("Got the ownership of {}", s2);

    // okay now in this way a function can take ownership of a value and return the ownership back to previous owner
    // by returning the same value. And also it can return the ownership of some value defined inside the scope of that
    // function.

    // Rust functions can also return multiple values by returning a tuple containing those values.

    let (name, surname) = returns_full_name();

    println!("My full name: {} {}", name, surname);

    /*
        While this works, taking ownership of a value and returning ownership with every function is a bit
        tedious. What if we want to let a function use a value but not take ownership?
        see lesson_13
    */
} // here x goes out of scope, then s. But because s's value was moved, nothing special happens

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("Just got the ownership of the value : {}", some_string);
} // here some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn returns_ownership(some_string: String) -> String {
    println!(
        "Just return the ownership back to the owner: {}",
        some_string
    );
    some_string
}

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("Just created a copy of the value : {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens

fn returns_full_name() -> (String, String) {
    let name = String::from("Abhishek");
    let surname = String::from("Prajapati");
    return (name, surname);
}

fn subtract(max: i64, min: i64) -> i64 {
    return max - min;
}

pub fn rust_functions() {
    // functions are something that holds a block of code to be executed when needed in the program/application/software.
    // the comments are also in a function.
    // functions in rust are similar to functions in any other programming language.
    // they can either take argument(s) or not.
    // they can either return a value or not.

    // one function can call another function inside it doesn't matter if the function was defined in the same scope or somewhere else.

    // define a function
    fn greet() {
        println!("hi !!");
    }

    // call it
    greet();

    // function with arguments
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }

    // call the function and store the returned value in a variable.
    let sum: i32 = add(3, 4);

    println!("{sum}");

    // above function were defined in the same scope.

    // this function subtract is defined outside of this scope.
    let result = subtract(10, 5);
    println!("{result}");
}

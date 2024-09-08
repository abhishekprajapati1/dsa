pub fn constants() {
    // we declare a constant variable by using the const keyword in rust
    const PI: f32 = 3.14;

    // convetion says to have all capital letter in the name of a constant variable. And have an underscore as a seperator
    // if there thre are more than one word.

    const PLAYER_SCORE: u32 = 32;

    println!("{}", PLAYER_SCORE);

    // though variables in rust are immutable by default but they can be marked as mutabe by mut keyword
    let mut age: i8 = 21; // generates a warning that it is never used may be have changed
    age = 34; // and since it is mutable we can safely do this
    println!("{}", age);

    // but a varibale declared using const keyword, becomes truly immutable forever
    // PI = 22/7; // uncomment to get error [invalid left-hand side of assignment]

    // we cannot even shadow a const variable. If we do it throws error.
    //let PI:i8 = 22/7; // uncomment to see the error [mismatched types]

    println!("{}", PI);

}

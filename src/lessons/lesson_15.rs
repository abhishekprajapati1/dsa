pub fn data_type_string() {
    // Going through this lesson will enhance our understanding of ownership.
    // But there is a question why we need another type for storing string values at all? We can use &str for the same purpose.

    // rust: String literals are convinent, but they aren't suitable for every situation in which we may want to use text.
    // One reason is that they're immutable. Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it?
    // For these situations, Rust has a second string type, "String". This type manages data allocated on the heap and as such is able to store an amount of text that is unkonwn to
    // us at compile time

    let mut name = "Abhishek";
    println!("{}", name);

    name = "Prajapati";
    println!("{}", name);

    // I am still not getting the need why we cannot assign name to a user input.
    // We can mark it mutable and then assign to the user input.
    // It should work this way. :(

    // anyways lets move and see what rust has to offer.
    let new_string = String::from("Abhishek Prajapati");
    // The double collon :: operator allows us to namespace this particular "from" function under the String type rather than using some sort of name like "string_from".
    println!("{}", new_string);

    // this kind of string can be mutated.
    let mut another_string = String::from("Amar");
    another_string.push_str("Rana"); // push_str appends a literal to a string.
    println!("{}", another_string);

    // OK; I think I got it we cannot update a string literal we can reassign it to another value.
    // but we can update a String object created in heap.

    // Every data stored in heap should be destroyed once it's owner goes out of scope.
    // So rust calls a "drop" function automatically for owners once their scope's closing
    // curly braces are reached.

    // NOTE: In c++, this pattern of deallocating resources at the end of an item's lifetime is
    // called a resource acquisition is initialization (RAII) pattern. The drop function in rust
    // will familiar to you if you've used RAII patterns.

    // NOTE: Rust also has a macro named "format!" that works like println! but instead of printing to the console, it returns a String value.

    // Mulitple variables can interact with the same data in different ways in Rust.
    let x = 5;
    let _y = x;

    // we can probably guess what this is doing: "bind the value 5 to x; then make a copy of
    // the value in x and bind it to y." We now have two variables, x and y, and both equal 5
    // This is indeed what is happening, because integers are simple values with a know fixed
    // size, and these two 5 values are pushed onto the stack.

    // Now let's look at the string version of it
    let s1 = String::from("Hello");
    let _s2 = s1;

    // This looks very similar, so we might assume that the way it works would be the same
    // : that is, the second line would make a copy of the value in s1, and bind it to s2.
    // But this isn't quite what happens.

    /*
        The string is stored in heap. And kind of meta information about that location and data
        is stored in the stack. Because working in stack rather than heap is more faster. There is three things are stored
        in this for s1 : 1. a pointer to the location in heap. 2. size of the string. 3. capacity of the allocated space in heap memory.

        Now when we did let s2 = s1;
        Actual data remains unchanged. Rust just copies the meta information in stack and creates another block named s2.
        which again contains 3 information: 1. a pointer to the location in heap. 2. size of the string. 3. capacity of the allocated space in heap memory.

        This way we didn't copied the actual physical data in memory. So that is unchanged. and both pointers from s1 and s2
        are pointing to the same location in heap.


        If Rust instead copied the heap data as well, the operation like s2 = s1 could be very
        expensive in terms of runtime performance if the data on the heap were to large.

        Earlier, we saw that when a variable goes out of scope, Rust automatically call the drop function
        and cleans up the heap memory for that variable. But in this case both s2 and s1 point to the same location.
        This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory.
        This is known as "double free error" and is one of the memory safety bugs.

        Freeing memory twice can lead to memory corruption, which can potentially lead to security vulerablilities.

        To ensure memory safety, after the line let s2 = s1; , Rust considers s1 as no longer valid.
        Therefore, Rust doesn't need to free anything when s1 goes out of the scope.
    */

    // so if you try to access s1 here. You'll get an error because rust prevents you from using the invalidated references.
    // println!("{}", s1);

    /*
        If you've heard the terms shallow_copy and deep_copy while working with other lanugage, the concept of copying the pointer, length and capacity
        without copying the data probably sounds like making a shallow copy. But because Rust also invalidates
        the first variable, instead of being called a shallow copy, it's known as a move.
    */

    /*
        There are times when we actually want to copy heap data of the String, not just stack data, we can use a
        common method called clone.
    */

    let s3 = String::from("I am IronMan");
    let s4 = s3.clone();

    println!("s1 = '{}', s2 = '{}'", s3, s4);

    // so far so good.
    // There is another wrinkle we haven't talked about yet.

    let x = 5;
    let y = x;

    println!("x: {}, y: {}", x, y);

    /*
        But this code above seems to contradict what we just learned: we don't have a call to clone,
        but x is still valid and wasn't moved into y.

        The reason si that types such as integers that have a known size at compile time are stored
        entirely on the stack, so copies of the actual values are quick to make. That means there's no reason
        we would want to prevent x from being valid after we create the variable y.
        In other words, there's no difference between deep and shallow copying here, so calling clone
        wouldn't do anything different from the usual shallow copying, and we can leave it out.
    */

    // does that means we could use clone here as well?
    let a = 5;
    let b = a.clone();
    println!("x: {}, y: {}", a, b); // well this is interesting and since it doesn't change anything we can avoid it...



}

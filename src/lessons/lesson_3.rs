pub fn data_types_and_integers() {
    // Rust is a statically typed language it means it should know the type of data being stored in some variable.
    // most time rust infers the type based on initialization or uses of the variable.
    let age = 32;
    println!("This is valid variable {}", age);

    // but some times we need to explictly tell the compiler the type of data. for example.
    let _age: i8 = "34".parse().expect("Not a number !"); 
    // [type annotations needed]
    // if we remove the type annotation from above statement. rust will throw an error. It means rust needs more
    // information from us to know what data is gonna be stored in shadowed age variable.

    // there are two subsets of data types in rust
    // Scalar Types => integers, floating-points, boolean, and characters
    // Integers
    // an integer is a number without a fractional component.
    let mark: i8 = 32; let passing_mark: u8 = 34;
    // i8 means it is signed integer that holds 8bit integer value
    // u8 means it is integer that holds 8 bit unsiegned integer value
    println!("See this signed {} and unsiegned {}", mark, passing_mark);

    // there are followin type of integer we can form based on bits
    /*
        i8  and  u8
        i32  and u32
        i64  and u64
        i128 and u128
        isize and usize
     */

    // as you can see each variant has two types i ie. signed and u ie. unsigned.
    // simply put if a value safely can be assumed as positive it is better to use u types of integer
    // but if signed actually matters for example +30 or -30 it is required to use i types of integer data type.

    
    // each signed iteger can store values in the range -(2^n - 1) inclusive to (2^n-1) exclusive. Where n is the number of bits.
    // so i8 will store values in range -128 to 127

    // let age: i8 = -129; // uncomment to get the error. :)
  
    // there is one more integer type that is isize and usize
    // it depends on the architecture of the device you are using
    // if you are using 32-bit architecture it will become i32 and u32
    // if you are using 64-bit architecture it will be i64 and u64
    let my_roll_no: isize = 323;
    println!("see this {}", my_roll_no);

    // In rust number literals can be bind with their types
    let age:i8 = 43i8; // and this is 
    // by default valuds are infered with types by compiler but if we mark a value with different type it throws an error
    //let age:i8 = 32i32; // uncomment to get error.

    // we can also use seperator in number literals to make it more readable
    let mut money:i32 = 10000;
    money = 10_000; // this is valid and both values are 10000 :)
    println!("See my money {}", money);

    // NOTE: In most cases you know what to store and what type it needs but if you are unsure leave it to default which is i32
    // this is suitable for most cases.

    // NOTE: Also the primary use case of isize and usize is indexing some sort of collections in system.

    println!("This is explicitly defined type {}", age);
}
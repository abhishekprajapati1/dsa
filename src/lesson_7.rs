// compound types can group multiple values into one type. Rust has two primitive compound types
// arrays and tuples

pub fn data_types_tuples() {
    // A tuple is a general way to grouping togather a number of values with a veriety of types into
    // one compound type.

    // Tuple have a fixed length: once declared, they cannot grow or shrink in size

    let my_tuple: (i32, f32, u8) = (400, 213.2, 233); // round braces are used & explicit typing...

    let my_tuple_2 = (23, 3, 3.23, -32); // implicitly inferred types.

    // to print the values we can used positional destructuring...

    let (x, y, z, k) = my_tuple_2;

    // here the types of the variables x, y, z and k are inferred by default.

    println!("see these values {} {} {} {}", x, y, z, k);

    // we can also use . operator to access the value of a tuple we need to put the index of the value after the dot (.)
    println!("see dot operatore in action: {}", my_tuple.0);

    // A tuple without any value is called unit in rust
    let _my_tuple_3 = (); // this is a unit. and its type is also a unit.

    // unit means => () just this pair of braces.

    // in above unit expression the type was inferred.
    // but we can explictly annotate this
    
    let _my_tuple_4: () = (); // this is now explicitly created unit;

    
    

}

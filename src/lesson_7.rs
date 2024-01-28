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

    // to access a tuple valud we can use dot (.) notation
    let new_tuple: (i32, i32, i32) = (3, 3, 4);

    println!("see this is value - {} is access using dot", new_tuple.2); // do not confuse with array... you cannot access that like new_tuple[2]
}

pub fn data_types_array() {
    // another way to have collection of multiple values is with an array.
    // array cannot hold multiple values with different data types
    // items of an array should be of same data types.
    let _a = [2, 3, 4, 2, 3]; // type is inferred to be a i32 array

    // we can define type of an array explicitly like below
    let _b: [i32; 3] = [2, 3, 2]; // in rust arrays are of fixed length

    // arrays are useful structure if you know the collection is not gonna change in size
    // for example name of months in a year
    let _months: [&str; 12] = [
        "January",
        "Fabruary",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // In rust we can initialize an array to have same value for each element
    let _same_in_general: [i32; 6] = [2, 2, 2, 2, 2, 2];
    let _same: [i32; 6] = [2; 6];
    // here both same_in_general and same are exactly same.


    // we use bracket notation with indices to access the value of an array
    println!("see this value {} accessed using bracket notation", _same[3]);

    // the program panicks if you try to access a value at an index out of the array limit
    // println!("error prone {}", _same[6]); // error: index out of bound; as we have the length 6 which will make upper limit to be 5
    // println!("error prone second {}", _same[-1]); // also we cannot use a negetive number as index for an item.


    // so to access the last element we can make use of len() method of arrays
    println!("see the last element {}", _same[_same.len() - 1]);
}

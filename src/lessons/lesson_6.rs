pub fn data_types_character() {
    // rust's char type is the language's most primitive alphabatic type.
    let _grade = 'A'; // inferred type
    let _my_grade: char = 'Z'; // with explicit typing.

    let the_cat: char = '😻';

    // IMPORTANT: char values should be wrapped inside single quotes and must have only one length.
    // let firstname_surname = 'AP'; // will cause panic.

    println!("I am a cat - {}", the_cat);

    // char data type takes 4 bytes in memory and represent a Unicode scalar value
    // which means it can a lot more than just ASCII


    // Accented letters; Chinese, Japanese, and Korean character; emoji; and zero with spaces; are all valid char values in rust
}

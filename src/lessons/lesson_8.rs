/*
    We will see a glimpse of rust's memory safety mechanism.
*/

use std::io;

pub fn get_month_name() {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
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
    let mut month_index = String::new();

    println!("Please enter the month index: ");
    io::stdin()
        .read_line(&mut month_index)
        .expect("Failed to read line");

    let month_index: usize = month_index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let monthname: &str = MONTHS[month_index];

    println!(
        "You entered the index: {}.\nAnd it indicates to month {}",
        month_index, monthname
    );

}


/*
    the above function compiles successfully. And workds great.
    BUT: if you enter a value that is not between 0 to 11, the program panicks and exit.

    EXPLAINATION:
    The program resulted in a runtime error when we used an invalid value to access the month.
    When you attempt to access an element of an error using index, rust checked if the specified value for index
    was less than the length of the array. If the value is greater than or equal to the length, rust panicks. This
    check has to happen at runtime, especially in this case. because the compiler can't possibly know what value
    a user will enter when they run the code later.


    This is an example of rust's memory safety principles in action. In many low level languages this kind of check
    is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against
    this kind of error by immediately exiting instead of allowing the memory access and continuing...
*/
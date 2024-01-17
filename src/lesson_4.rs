pub fn floating_point_numbers() {
    // In rust there is two types are available
    // 1. f32 and f64. default is f64
    // All floating point numbers are signed so there is no concept like i and u :)

    let marks = 3.2; // f64 bcause my machine is of 64 architecture. f32 values are single precision

    let maximum_marks = 23.43; // f64 f64 values are dynamic precision


    println!("{} {}", marks, maximum_marks);

    // we can do all basic math operation on these data types
    // addition
    let sum = 12 + 32;
    println!("{}" , sum);

    // subtraction
    //let subtract = 45 - 3.2; // this is gonna generate error
    let stubtract_result  = 45f32 + 3.2f32; // however this did the trick
    println!("{}", stubtract_result);

    // it means rust cannot assume the data. we will have tell it what data we are dealing with.

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

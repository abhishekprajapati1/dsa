pub fn match_in_rust(){
    // match expresions are a powerful tool in Rust for pattern  matching and branching based on different patterns
    // They are often used to replace if else chains.
    let myage = 78;

    match myage {
        10 => println!("No you are wrong"),
        20 => println!("Yes that's correct."),
        _ => println!("Nothing matched.")
    }

    // we can also match ranges like below
    match myage {
        0..=5 => println!("The value is between 0 and 5."),
        6..=10 => println!("The value is between 6 and 10"),
        11..=50 => println!("The value is between 11 and 50"),
        _ => println!("The value is not found in the specified ranges"),
    }

    // we use match to match tuples.
    let pair = (0,2); // defining a tuple.
    match pair {
        (0, 0) => println!("The co-ordinates are origin."),
        (0, _) => println!("The co-ordinates first value is 0."),
        (_, 0) => println!("The co-ordinates second value is 0."),
        _ => println!("The co-ordinates are not special"),
    }

    // we can match custom data shapes too
    enum Color {
        Red,
        Green,
        Blue,
        Yellow,
    }

    let color = Color::Green;

    match color {
        Color::Red => println!("Stop riding."),
        Color::Blue => println!("Can't you see the sky."),
        Color::Green => println!("Start riding."),
        _ => println!("Do whatever you want."),
    }
}

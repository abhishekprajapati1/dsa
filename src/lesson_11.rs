pub fn expression_statement() {
    // in rust there is a notable differece between statement and expression.
    // and they behave differently.

    let _y = {
        let _x = 32;
    };

    // above block of code is assined to y. but y will get a value only and if only the block evaluates to a value.
    // in above case this block does not evaluates to a value. it just assigns 32 to a block scoped variable x.

    // if we try to print the y, we get an error that y cannot be formatted as it doesn't have a value.
    // println!("{}", y);


    // if you are wondering what is in y for the moment, it is a unit ie. empty tuple.
    // and this is why I guess that we cannot format a collection in string. that is why the error was generated during the println macro.

    // now lets' make the block evaluate to a value.
    let z: i32 = {
        let y: i32 = 32;
        y + 4 // this is a value unlike the first line of this block what is an assignment hence it is a statement.
    };

    println!("{}", z); // prints 36 on terminal


    // it means following should be a valid function

    fn add(a:i32, b:i32) -> i32 {
        a+b
    }

    println!("{}", add(4, 3)); // and it is... prints 7 in terminal


    // here comes the intersting point that if you put a semicolon after the last expression
    // it becomes a statement. :)

    let _n: i32 = {
        let y: i32 = 32;
        y + 4
    };

    // above code is not valid as it does not return a value so a unit is assigned to n and the whole code becomes a error prone.
    

}

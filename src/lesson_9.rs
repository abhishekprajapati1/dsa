pub mod control_flow {

    pub fn ifelse() {
        // if-else is as basic as in other programming language.
        // it is more like python's if-else that doesn't require parentesises.
        let mut age: i32 = 12;

        // in rust if expressions' conditions are sometimes called "arms"
        if age >= 18 {
            println!("You are an adult.")
        } else {
            println!("You are still a kid.")
        }

        age = 21;

        if age >= 18 {
            println!("Now you are an adult.")
        } else {
            println!("You are not gonna grow.")
        }

        // chain goes like blow
        const NAME: &str = "abhishek";

        if NAME == "dileep" {
            println!("Hello dileep.");
        } else if NAME == "abhishek" {
            println!("Hello abhishek");
        } else {
            println!("You are not registered.");
        }

        // one more thing if you are from js background let me tell you that value as expression is not valid.
        // so expression must evaluate to true else you are gonna get a panic.

        // we can also use an if-else flow to a variable like below
        const CONDITION: bool = false; //
        let new_age: i32 = if CONDITION { 18 } else { 21 };

        // do not mistakenly assume it to be a shorthand for if-else.
        println!("see this new age - {}", new_age); // I just found that semicolons at the end of line are optional :P
                                                    // but it only if it is the last statement of the block........:0

        // can we have multiple conditions in shorthand ???
        let condition_2: bool = false;

        let new_age_2: i32 = if CONDITION {
            18
        } else if condition_2 {
            21
        } else {
            32
        };

        println!("{}", new_age_2);

        // while assigning variables with ifelse chain we cannot assing different types to a single variable.
        //let dynamic_var = if true {4} else {"abhishek"}; // this will cause rust to panic. we cannot do that.
    }

    pub fn rustloops_loop() {
        // looping is process to repeat the execution of some code block untill a condition becomes false.
        // rust provides several way to loop through a code block.

        // 1. loop:

        let mut count: i32 = 0;

        // loop {
        //     println!("number - {}", count);
        //     count = count + 1;
        // }

        // the above commented block will runs for infinte times as we didn't mention a condition to stop it.

        // loop keyword is useful when you want to return a value from the loop
        let last_value: i32 = loop {
            count = count + 1;
            if count == 10 {
                break count * 2; // the value after break is optional think the break as return statement for loop
            }
        };

        print!("{}", last_value);

        /*
          Rust compiler treats a break expression and a return expression as having the value unit, or ().
          remember unit from tuple chapter a unit tuple is an empty tuple.
        */


        // loop lables:

        /* 
            loop labels can be used to identifiy loops. this is usefull when you have multiple nested loop.
            in general if you specify break or continue in the innermose loop, the break and continue will
            take effect in the innermost loop right ?

            with labels you can use break and continue with the label and the break and continue will take
            effect in the loop whose label was used.
        */

        


    }
}

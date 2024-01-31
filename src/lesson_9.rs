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
        // so expression must evaluate to true else you are gonna get a panick.

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
    }

    pub fn whileloop() {
        println!("looping......")
    }
}

pub fn variables() {
    // variables can be created with let keyword along with their types; type is optional
    let number1: i32 = 3;
    // if we do not intialize a variable it will give a warning
    let a: i32;
    // println!("{}", a); // and if we try to use an un initialized variable then it will throw an error; un comment this line to see the error

    // we can suppress the uninitialized warning by using an underscore before the un initialized variable
    let _b: i32;

    // by default each variable is immutable
    //number1 = 32; // uncomment this to get error;

    // we can make a variable mutable by using mut keyword
    let mut mutable_number: i32 = 32;

    mutable_number = 23;

    // all the variables are block scoped following is the illustration for the same.
    let name: &str = "Abhishek";
    let surname: &str = "Prajapati";
    let mut freind_name : &str = "Praveen";
    {
        let name: &str = "Dileep"; // this is only accessible inside the current scope. this is called shadowing a variable. Can we do so in original scope also :)
        println!("Hello Mr. {}", name);
        let name: &str = "Chhuii Muii"; // shadowing in original scope
        println!("Hello umm.. {}", name); // I am surprised why I'm not getting any error... this is strange...

        // this scope has access to its parent. so if a variable is not defined in this block scope it will try to search that in its
        // parent block's scope
        println!("Hello Mrs. {}", surname);

        // but what will happen when we try to mutate variable that is defined in parent block's scope. ?
        freind_name = "Dileep";
        println!("Tried changing the name {}", freind_name); // as expected we are getting mutated value here...
    }

    // what if we print the friend name here in outer scope, is it changed ???
    println!("The effect in outer scope {}", freind_name); // it is affected. The change is global :)

    println!("Hello Mr. {}", name);

    println!("See this number: {}", mutable_number);
}

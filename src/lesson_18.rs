pub fn rust_loop_loop () {
  // loop is a special keyword in rust language. Helps running a block of code infinitely until
  // specifically broken by break keyword. This can be useful for tasks that need to repeat
  // indefinitely or until a certain condition is met.

  let mut counter = 1;

  loop {
      if counter > 10 {
          break;
      }
      println!("{}", counter);
      counter += 1;
  }

  // why rust has this. when we can achieve the similar behaviour with while loop?

  /*
    1. "loop" loop requires explicit break statement to quit the loop but while loop can break once a condition becomes false.
    2. "loop" loop is inherently infinite. This can be useful for tasks that need to run continuously, such as server processes,
        or game loops. While we can create infinite loops using while keyword, the loop keyword makes the intent more clear.
    3. Rust allows you to label "loop" blocks. This is not possible with while loops.
    4. "loop" can return a value when break;
  */

  // labeling
  'outer: loop{
      println!("Outer loop iteration");
      'second_outer: loop {
          println!("Second level outer loop iteration");
          loop {
              println!("Inner loop iteration");
              break 'outer // since we used label so the outer loop will break even if break is used inside innermost loop.
          }
      }
  }

  // return values;
  let mut age = 10;
  let new_age = loop {
      age += 1;
      if age == 18 {
          break age; // this will return the age and age will be assigned to new_age variable.
      }
  }; // this semicolon is required because loop {} here is a statement.

  println!("New age is {}", new_age);
}

// As mentioned in Chapter 2, by default variables are immutable. This is one of many nudges Rust gives you to write your code in a 
// way that takes advantage of the safety and easy concurrency that Rust offers.
// However, In Rust we can also make our variables to be mutable.

// When a variable is immutable, once a value is bound to a name, you can’t change that value.
// In other words, Rust works just like Haskell.

fn main(){
  let x = 5;
  println!("The value of x is: {}", x);
  x = 6;                                //This code will not compile because I am trying to change an immutable variable...
  println!("The value of x is: {}", x); 
}

// But mutability can be very useful. Variables are immutable only by default; 
// To do that, just use the keyword "mut" before the name of the variable.
// In addition to allowing this value to change, "mut" conveys intent to future readers of the code by indicating that other parts of the
// code will be changing this variable’s value.

fn main(){
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 23;
  println!("The value of x now is: {}", x);
}

// There are multiple trade-offs to consider in addition to the prevention of bugs. For example, in cases where you’re using large data
// structures, mutating an instance in place may be faster than copying and returning newly allocated instances. With smaller data 
// structures, creating new instances and writing in a more functional programming style may be easier to think through, so lower 
// performance might be a worthwhile penalty for gaining that clarity.

//We also are allowed to declare a variable (be it mutable or immutable) and not assign an initial value to it...
//For example, we can do the following:
fn main(){
  let mux a;
  a = 17;
  println!("the value of a is: {}.", a);
  a = 234;
  println!("now, the value of a is: {}.", a);
}



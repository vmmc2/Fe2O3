//Rust is a statically-typed language. This means that it must know the type of all variables at compile-time
//The Rust Compiler, as well as the GHC, can usually infer what type we want to use based on its value and how we use it.

//Scalar Type:
// A scalar type represents a single value. Rust has four primary scalar types: booleans, characters, integers, floating-point

//4) Characters
/*
- Characters (or just char) is the way of representing is the most primitive alphabetic type.
- In Rust, when dealing with characters we use single quotes ('') but when we are dealing with string, we use double quotes ("").

- In Rust, the 'char' type has a size of 4 bytes (32 bits) and represents a Unicode Scalar Value, which means that a character in Rust can represent a lot more than
just a ASCII value.
- By a lot more, we mean: Accented letters, chinese, japanese and korean characters. Oh.. and emojis too.
- To be more specific, when we are dealing with Unicode Scalar Values we are dealing with values that are in the ranges below:
  I) U+0000 to U+D7FF
  II) U+E000 to U+10FFFF, inclusive.
- Also, we have a type annotation for characters too in Rust: we use the keyword 'char'.
*/

fn main(){
    let number: u32 = 127;
    let c: char = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    
    println!("The value of the variable c is: {} and the value of number is: {}", c, number);

}

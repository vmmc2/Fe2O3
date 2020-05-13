//Rust is a statically-typed language. This means that it must know the type of all variables at compile-time
//The Rust Compiler, as well as the GHC, can usually infer what type we want to use based on its value and how we use it.

//Compound Types: Different from scalar types, compound types are types of data that can store/group multiple values inside just one type. Rust has two basic compound types: Arrays and Tuples.
//Em outras palavras, a gente tem varios dados que podem ser armazenados/guardados em um unico tipo.

//6) Tuples:
/*
- A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
- By the description provided above, tuple works similar to structs from C/C++.
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- We create a tuple by writing a comma-separated list of values inside parentheses. 
- Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
- We are can also put annotations when we are declaring the tuple. But this is optional. For more info, look at the example provided inside the main() function.


*/

fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //In the example above we have a tuple of three elements. The types are: signed int of 32-bits, floating-point number of 64-bits, unsigned int of 8-bits.
    
    //To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
    let (x, y, z) = tup;
    println!("The value of y is: {}", y); //it's going to print: 6.4
    
    //It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z. 
    //This is called destructuring, because it breaks the single tuple into three parts. Finally, the program prints the value of y, which is 6.4.
    
    //In addition to destructuring through pattern matching, we can access a tuple element directly by using a period (.) 
    //followed by the index of the value we want to access. 
    //As with most programming languages, the first index in a tuple is 0.
    let x: (i64, f32, i64) = (500, 6.4, 1);
    let mut five_hundred = x.0;
    let six_point_four = x.1;
    let mut one = x.2;
    
    
    
}

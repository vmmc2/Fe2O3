//Functions...

//As you have already seen, in Rust we have the main() function, which is the entry point of every program written in Rust.
//By this, we mean that every time that we start running a program in Rust, its execution starts in the main() function.

//In Rust, the keyword to declare a function is: "fn"
//The convention for declaring a function in Rust is different from the convention that we use when we are declaring a function in Dart, for example. 
//In Rust, we must use only lowercase letters and different words are separated by underscore. 
//Take a look at the example below:

fn another_function(){
    println!("Yet another Hello World!");
}


fn main(){
    println!("Hello World!");
    
    another_function();
}

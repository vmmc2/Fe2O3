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


fn yaf(number: i32){ //A function that has a parameter defined inside its signature.
    println!("The value of the number variable is: {}", number);   
}

fn fibonacci(x: u32) -> u32{
    if x == 1 || x == 2{
        return 1;
    }else{
        return fibonacci(x - 1) + fibonacci(x - 2);
    }
}

fn main(){
    println!("Hello World!");
    
    another_function();
    println!("");
    yaf(69);
    println!("");
    println!("{}", fibonacci(10));
}

//After declaring the function name in Rust, we have a set of parentheses. If that's the case, we are going to put the parameters of our function inside the parentheses.
//And after that, we have a set of curly brackets. This set of curly brackets tells where the function begins and where the function ends.
//Unlike C, Rust doesn’t care where you define your functions, only that they’re defined somewhere in your code.
//In function signatures, you must declare the type of each parameter.  This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler 
//almost never needs you to use them elsewhere in the code to figure out what you mean.

//Rust is a statically-typed language. This means that it must know the type of all variables at compile-time
//The Rust Compiler, as well as the GHC, can usually infer what type we want to use based on its value and how we use it.

//Scalar Type:
// A scalar type represents a single value. Rust has four primary scalar types: booleans, characters, integers, floating-point

//1) Integers:
/*
- An integer is a number without a fractional component. (Eh um numero sem casas decimais ou parte fracionaria).
- We have several different types anotations for integers in Rust. These type annotations are divided in two major groups: 'u' that stands for 'unsigned' and 'i' 
that stands for 'signed' integer.
- Take a look at the table below for more info:

Length  ------- Unsigned ------- Signed
8 bits             u8              i8
16 bits           u16             i16
32 bits           u32             i32
64 bits           u64             i64
128 bits         u128            i128
arch            usize           isize

- Signed and Unsigned refer to whether is possible for the number to be positive or negative. In other words, whether the number needs to have a sign with it.
- It is like to write numbers in a paper. In case it's possible to have positive and negative numbers we are going to write the number with a positive or a negative
sign. However, when it's safe to assume that the number is always positive, it's shown no sign at all.
- In Rust, signed numbers are stored using the two's complement representation.
- What is the range of each variation?
  Unsigned (n bits) => 0 to 2^(n) - 1, inclusive.
  Signed   (n bits) => -(2^(n - 1)) to 2^(n - 1) - 1, inclusive.
  
- Additionally, there are also another two types: 'usize' and 'isize'. These two other variants depend on the type of computer that your program is running on:
64 bits if we are running on a 64-bit machine or 32 bits if we are running on a 32-bit machine.
- To write number literals we can always use '_' to make it better for us to read.
- i32 is generally faster than i64, even if we are on a 64-bit machine.
*/

//2) Floating-Point
/*
- Rust has also two different options when we are dealing with Floating-Point numbers. The first one is to use 32-bits Floating-Point. The second one is to use
64-bits Floating-Point.
- The type annotations are: f32 (32-bits) and f64 (64-bits).
- The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
- Floating-point numbers are represented according to the IEEE-754 standard. 
- The f32 type is a single-precision float, and f64 has double precision.
*/


fn main(){
    let a: u32 = 1_000_000;
    let b: u8 = 0b1001_0000;
    println!("The value of a is: {}!", a);
    println!("The value of b is: {}!", b);
    
    let pi = 3.1415; // f64 because it is the default type.
    let mut x: f32 = 2.34; //we must say that it is a 32-bits floating-point number, otherwise it will be declared as a 64-bits floating-point number by default.
    
    //We can also be explicit about the 64-bits floating-point type. We just need to declare it as stated below:
    let euler_constant: f64 = 2.71;
    println!("The euler constant has a value of {}", euler_constant);
    
    //The example below show to us how to use the remainder operator (o operador resto).
    let mut c = 4 % 3;
    println!("O valor de c: {}", c);
    
    //Divisao inteira com Rust: Nao arredonda. Apenas descarta a parte decimal.
    let q: i32 = 5/3;
    println!("{}", q);
    
}

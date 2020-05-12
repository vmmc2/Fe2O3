//Like immutable variables, constants are a programming concept in which values are bounded to a name and are not allowed to change.

//But there are some differences between "immutable variables" and "constants":
//1) You are not allowed to use the "mut" keyword with constants. Constants are always immutable.
//2) You declare a constant by using the "const" keyword instead of the "let" keyword, and the type of the value must be annotated. In other words, you must specify what is the type of the constant that you are declaring.
//3) Constants can be declared in any scope inside your code.
//4) The last and, perhaps, the most important difference is that constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

const MAX_POINTS: u32 = 10_000;
// Declared the constant using the "const" keyword, also annotated its value's type and set it to a constant expression.


fn main(){
  println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

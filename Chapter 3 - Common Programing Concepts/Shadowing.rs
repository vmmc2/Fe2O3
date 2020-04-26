//You can declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable.
//And we can do this without using a mutable variable.
//Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what appears when the variable is used.
//We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows:

fn main(){
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println("The value of x is: {}.", x);
}

//Shadowing is a different thing when compared to using a mutable variable. When we use "let" (shadowing) we are creating a new variable.
//The other difference between mut and shadowing is that because we’re effectively 
//creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
//It is a concept really similar to what we do in Haskell.

let spaces = "   ";
let spaces = spaces.len();

//This construct above is allowed because the first spaces variable is a string type and the second spaces variable, which is a 
//brand-new variable that happens to have the same name as the first one, is a number type.

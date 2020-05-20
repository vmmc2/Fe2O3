//1) What is Ownership??
/*
- It is Rust's central feature.
- It has deep implications in many aspects of the language.
- First of all, we must understand what are the approaches to manage memory while a program is running.
  * The first approach is to use a garbage collector (GC): a garbage collector is a tool that is constantly looking for no longer used memory as the program runs (Java).
  * The second approach is where the programmer himself has to deal with memory management. He has to explicitly allocate and free the memory (C).
  * The third approach is the one that Rust uses: memory is managed through a system of ownership with a set of rules that the compiler checks at 
  compile time. None of the ownership features slow down your program while it’s running.
- Because ownership is a new concept for many programmers, it does take some time to get used to. The good news is that the more experienced 
you become with Rust and the rules of the ownership system, the more you’ll be able to naturally develop code that is safe and efficient. Keep at it!

1.1) The Stack and the Heap.
- Usually, when we are dealing with programming languages we do not need to think about whether our variables are being stored in the Stack or in the
Heap. But when we are working with a systems programming language we have to worry about it because we will have to do decisions about it.
- Let's talk about it:
- Both the Stack and the Heap are parts of the memory that our code can use at runtime but these parts of the memory are structured in different ways.
- The Stack stores values in the order it gets them and removes the values in the opposite order. It works following the "last in, first out" policy.
Adding data is called "pushing onto the Stack", removing data is called "popping off the Stack".
- All data stored on the stack must have a known, fixed size.
- Data with an unknown size at compile time or a size that might change during the runtime must be stored on the Heap instead.
-  The Heap is less organized: when you put data on the Heap, you request a certain amount of space. The operating system finds an empty spot 
in the Heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called 
allocating on the heap and is sometimes abbreviated as just allocating.
- Pushing values onto the stack is not considered allocating.
----------------------------------------------------------------- IMPORTANT -----------------------------------------------------------------------
- Pushing to the Stack is faster than allocating on the Heap because the operating system never has to search for a place to store new data. 
That location is always at the top of the stack. Comparatively, allocating space on the Heap requires more work, because the operating system 
must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

---------------------------------------------------------------------------------------------------------------------------------------------------
- When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and 
the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.


------------------------------------------------------------ UNDERSTANDING OWNERSHIP --------------------------------------------------------------
1) Each value in Rust has a variable that is called its "owner".
2) There can only be one "owner" at a time.
3) When the "owner" goes out of scope, the value will be dropped.
*/


fn main(){
    //A scope is the range within a program for which an item is valid
    //When s comes into scope, it is valid.
    //It remains valid until it goes out of scope.
    let mut s = "hello";
    println!("{}", s);
    //We’ve already seen string literals, where a string value is hardcoded into our program. String literals are convenient, 
    //but they aren’t suitable for every situation in which we may want to use text. 
    //One reason is that they’re immutable. You cannot change them even if we use the "mut" keyword.
    
    //Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it? 
    //For these situations, Rust has a second string type, String. 
    //This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
    
    //You can create a String from a string literal using the from function, like so:
    //This kind of string can be mutated:
    let mut a = String::from("Dale porra."); //The double colon (::) is an operator that allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.
    println!("{}", a);
    a.push_str(" Zildao god!");
    println!("{}", a);
}
















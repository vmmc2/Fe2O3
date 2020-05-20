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

//Difference between the String literal and the String that is store in the Heap.
/*
- In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.
- This is why string literals are fast and efficient.
- But these properties only come from the string literal’s immutability. 
- With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. 
- This means that:
  1) The memory must be requested from the operating system at runtime.
  2) We need a way of returning this memory to the operating system when we’re done with our String.
  
- That first part is done by us: when we call String::from, its implementation requests the memory it needs. 
This is pretty much universal in programming languages.
- However, the second part is different. In languages with a garbage collector (GC), the GC keeps track and cleans up memory that isn’t being
used anymore, and we don’t need to think about it. Without a GC, it’s our responsibility to identify when memory is no longer being used and 
call code to explicitly return it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. 
If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair 
exactly one allocate with exactly one free.
- Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. 
*/


//Ways that Variables and Data interact: Move
/*
- Multiple variables can interact with the same data in different ways in Rust. Look at the example below in which we use an integer:
let x = 5;
let y = x;
- We can probably guess what this is doing: “bind the value 5 to x; then make a copy of the value in x and bind it to y.” We now have two 
variables, x and y, and both equal 5. This is indeed what is happening, because integers are simple values with a known, fixed size, and 
these two 5 values are pushed onto the stack.


- Now, take a look at the string version of what was done above:
let s1 = String::from("hello");
let s2 = s1;
- This looks very similar to the previous code, so we might assume that the way it works would be the same: that is, the second line would 
make a copy of the value in s1 and bind it to s2. But this isn’t quite what happens.
- A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. 
This group of data is stored on the stack.
- The length is how much memory, in bytes, the contents of the String is currently using. The capacity is the total amount of memory, in bytes, 
that the String has received from the operating system. The difference between length and capacity matters, but not in this context, so for now, 
it’s fine to ignore the capacity.
- When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. 
We do not copy the data on the heap that the pointer refers to.
- Again, Rust does not copy the data heap when we do an assignment of type: s2 = s1;
- Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that 
variable. But Figure 4-2 shows both data pointers pointing to the same location. This is a problem: when s2 and s1 go out of scope, they will 
both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing 
memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
- To ensure memory safety, there’s one more detail to what happens in this situation in Rust. Instead of trying to copy the allocated memory, 
Rust considers s1 to no longer be valid and, therefore, Rust doesn’t need to free anything when s1 goes out of scope.
- Check out what happens when you try to use s1 after s2 is created; it won’t work.
- If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and 
capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead 
of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2.
*/


//Ways that Variables and Data interact: Clone
/*
- If we want to copy the heap data of the String (not only the data that is on the Stack(pointer,length,capacity)), we can use a method called "clone()".
- When you see a call to "clone()", you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.
- Take a look at the example below to see the "clone()" method in action:

let s1 = String::from("Dale");
let s2 = s1.clone(); //Now, the heap data does get copied.
println!("The content of the String s2 is: {}", s2);
*/

//Stack-Only Data: Copy
/*

*/








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


*/

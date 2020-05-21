//Reference and Borrwing

/*
- Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership 
of the value: (See the main() function and the calculate_length() function)
- Note that when we call the calculate_length() function, we pass the String as "&s" and in the declaration of the function, we also have this parameter:
s: &String.
- This ampersand(&) means that we are using references. They allow you to refer to some value WITHOUT taking ownership of it.
*/


/*
fn calculate_length(s: &String) -> usize{ //s is a reference to a String
    s.len() //Here I am returning the length (number of chars) of my String s
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
  
fn main(){
    let s1 = String::from("Hello World!");
    
    let len = calculate_length(&s1);
    println!("The length of {} is: {}.", s1, len);
}
*/


/*
- The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. 
- Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.
- Likewise, the signature of the function uses "&" to indicate that the type of the parameter s is a reference.
- The scope in which the variable s is valid is the same as any function parameter’s scope, but we don’t drop what the reference points to when it
goes out of scope because we don’t have ownership. 
- When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, 
because we never had ownership.
- We call having references as function parameters borrowing. As in real life, if a person owns something, you can borrow it from them. 
When you’re done, you have to give it back.
- So what happens if we try to modify something we’re borrowing? Spoiler Alert: It won't work. (We receive a compiler error).
- JUST AS VARIABLES ARE IMMUTABLE BY DEFAULT, SO ARE REFERENCES. WE'RE NOT ALLOWED TO MODIFY SOMETHING WE HAVE A REFERENCE TO.
- How can we avoid this problem?? See the code below:
*/


fn change(s1: &mut String){ //First of all, in the change() function declaration, we have to declare the &s1 reference as mutable.
    s1.push_str(". Tudo bem com vc??");
}
//First, we had to change s to be mut. Then we had to create a mutable reference with &mut s 
//and accept a mutable reference with s1: &mut String.
fn main(){
    let mut s = String::from("Oi, meu chapa"); //We have to make our String to be mutable also.
    println!("{}", s);
    change(&mut s); //We must pass our String as a mutable reference to the function also.
    println!("{}", s);
}
/*
- But mutable references have one big restriction: YOU CAN HAVE ONLY ONE MUTABLE REFERENCE TO A PARTICULAR PIECE OF DATA IN A PARTICULAR SCOPE.
*/

/*
- The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and 
happens when these three behaviors occur:
1) Two or more pointers access the same data at the same time.
2) At least one of the pointers is being used to write to the data.
3) There’s no mechanism being used to synchronize access to the data.

- Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; 
Rust prevents this problem from happening because it won’t even compile code with data races!
- As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones.
- We also cannot have a mutable reference while we have an immutable one. However, multiple immutable references are okay .
*/



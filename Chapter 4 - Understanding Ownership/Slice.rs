// The Slice Type

/*
- Function to clear a mutable String (mut): clear();
*/

/*
- Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection 
rather than the whole collection.
- As an example, take a look at the function below which has the purpose of returning the ending index of the first word in the string 
received.
- Let's take a look about how the function "first_word" works...
  1) It receives a reference to the string that was passed to the function as a parameter. Because of that, we don't need to worry about 
  issue involving "Ownership".
  2) Then, we convert the string to an array of bytes using the "as_bytes()" method: let bytes = s.as_bytes();
  3) Next, we create an iterator to iterate through our array of bytes by using the iter() method. For now, know that iter is a method 
  that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple 
  instead. The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element. 
  This is a bit more convenient than calculating the index ourselves.
  4) Because the "enumerate()" method returns a tuple, we can use patterns to destructure that tuple, just like everywhere else in Rust. So
  in the for loop, we specify a pattern that has i for the index in the tuple and &item for the single byte in the tuple. Because we
  get a reference to the element from .iter().enumerate(), we use & in the pattern.
*/

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

//String Slices
/*
- Fortunately, Rust offers to us a solution: The use of string slices. A string slice is no more than a reference to a part of a string.

*/

fn main(){
  let mut s = String::from("hello world");
  //Os String Slices funcionam exatamente como os ranges. A extremidade esquerda é inclusiva enquanto que a extremidade direita é exclusiva.
  let hello = &s[0..5];
  let world = &s[6..11];
  
}


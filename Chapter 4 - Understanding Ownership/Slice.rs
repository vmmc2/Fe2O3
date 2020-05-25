// The Slice Type

/*
- Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection 
rather than the whole collection.
- As an example, take a look at the function below which has the purpose of returning the ending index of the first word in the string received.
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

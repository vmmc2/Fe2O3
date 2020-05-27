//A comparison between using Tuples and using Structs to solve a problem.
//In this case, we must represent a rectangle and calculate its area (number of pixels that the rectangle occupies in the screen)

//Approach 1: Using Tuples

fn calculate_area(rect: (u32, u32)) -> u32{
    return (rect.0 * rect.1);
    //rect.0 -> width
    //rect.1 -> height
}

fn main(){
    let rectangle1: (u32, u32) = (30, 50); //first value of the tuple is the width and second one is the height.
    
    println!("The area of the rectangle is: {}.", calculate_area(rectangle1));
}

//This solution is good since it groups data that are related to each other (In this particular case, we are grouping "width" and "height").
//But, on the other hand, we lose in meaning. We must keep in mind that the 0-index is the width and the 1-index is the height of our rectangle.


//Approach 2: Using Structs

struct Rectangle{
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32{
    rect.width * rect.height
}

fn main(){
    let rec1 = Rectangle{
      width: 50,
      height: 30,
    };
    let rec2 = Rectangle{
        width: 14,
        ..rec1
    };
    println!("The height of rec2 is: {}.", rec2.height);
    println!("The area of this rectangle is: {}", area(&rec1));
    
    
}

// Control Flow

//Intro: Deciding whether or not to run some code depending on if a condition is true 
//and deciding to run some code repeatedly while a condition is true are basic building blocks in most programming languages.

//We usually do this by using 'if' expressions and loops.


//2) Loops
/*
- For this task, Rust provides several loops. A loop runs through the code inside the loop body to the end 
and then starts immediately back at the beginning (of the loop)
- Rust has three kinds of loops: loop, while, and for..

2.1) loop: The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop. 
- You can place the "break" keyword within the loop to tell the program when to stop executing the loop.
One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. 
However, you might need to pass the result of that operation to the rest of your code. 
- To do this, you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:

2.2) while:
- It’s often useful for a program to evaluate a condition within a loop. While the condition is true, the loop runs. 
When the condition ceases to be true, the program calls break, stopping the loop. 
This loop type could be implemented using a combination of loop, if, else, and break; you could try that now in a program, if you’d like.
- However, this pattern is so common that Rust has a built-in language construct for it, called a while loop. 
- See the example below for more info:
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

2.3) for:
- This type of loop is commonly used when we want to iteract through a collection of data/elements.
- You could iteract through an array for example using this dumb solution below:
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
- But using "for" loop is a much better option.
- We’ve now increased the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.
- The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
*/


fn trying_for(){
    let vetor: [i32; 4] = [10,20,30,40];
    let vetor2: [f32; 5] = [34.3423, 34.2321, 12.4, 3.1415, 2.71];
    for element in vetor.iter(){
        println!("The current element in the collection is: {}", element);
    }
    //Trying to do it in another way..
    for index in (0..5){ //eh que nem o range de python3: Vai de 0 ate 4
        println!("vetor2[{}] = {}", index, vetor2[index]);
    }
}

fn trying_loop(){
    let mut number: i32 = 1;
    loop{ //O bloco de codigo dentro de um "loop" fica executando indefinidamente ate que a gente interrompa o loop explicitamente usando a keyword "break"
        if number > 10{
            break;
        }
        println!("The value of number is: {}", number);
        number += 1;
    }
}

fn trying_loop2(){
    let mut counter: i32 = 0;
    let num: i32 = loop{
        if counter == 10{
            break counter * 5;
        }
        counter += 1;
    };
    println!("The value of num is: {}", num);
}

fn main(){
    trying_loop2();
    trying_for();
}

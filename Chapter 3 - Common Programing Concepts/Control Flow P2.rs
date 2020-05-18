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
You can place the "break" keyword within the loop to tell the program when to stop executing the loop.

One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. 
However, you might need to pass the result of that operation to the rest of your code. 
To do this, you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:
*/
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
}

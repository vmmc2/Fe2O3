// Control Flow

//Intro: Deciding whether or not to run some code depending on if a condition is true 
//and deciding to run some code repeatedly while a condition is true are basic building blocks in most programming languages.

//We usually do this by using 'if' expressions and loops.


//1) "if" expressions
/*
- An "if" expression allows you to branch your code depending whether a condition (or more than 1 condition) is true or false.
- You basically provide a condition and then state: If this condition is met (If it's true), then run this block of code. Otherwise, do not run this code of block.
- In Rust, everything that we put inside the condition of an "if" or "else if" statement must be a boolean. Otherwise, the code will throw an error.
- Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean.
- You must be explicit and always provide if with a Boolean as its condition.
- You can have multiple conditions by combining if and else in an else if expression. For example look to the function condition2()
- If we have an "if-else" block, as soon as one condition inside that block is met, it is going to be executed and the code will continue 
running outisde of that block, even if that are other conditions that are also true after the one that we first met.
- Or, in other words, Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.
- We can also use "if" statements with "let" keyword. See the assignment() function for more info.

*/

fn assignment() -> i32{
    let x: i32 = 16;
    let number: i32 = if x < 17 {4} else {8};
    return number;
}


fn main(){
    //condition1();
    //condition2();
    //condition3();
    
    let answer = assignment();
    println!("The value of asnwer is: {}", answer);
}

fn condition1(){
    let number: u16 = 30;
    
    if number < 40 {
        println!("The number is less than 40."); //It is going to execute this block of code since number is 30 and 30 is less than 40.
    }else{
        println!("The number is greater than or equal 40.");
    }
}

fn condition2(){
    let mut number: u32 = 40;
    
    if number % 5 == 0{
        println!("The number provided is divisible by 5.");
    } else if number % 3 == 0 {
        println!("The number provided is divisible by 3.");
    } else if number % 2 == 0{
        println!("The number provided is divisible by 2.");
    } else{
        println!("The number provided is not divisible by 5, 3 or 2.");
    }
}

fn condition3(){
    let num: i32 = 16;
    
    if num < 8{
        println!("The number is less than 8.");
    }else{
        println!("The number is greater than or equal to 8.");
    }
    
    if num % 4 == 0{
        println!("The number is divisible by 4");
    }
    if num % 4 != 0{
        println!("Deu ruim.");
    }else {
        println!("Deu bom.");
    }
}

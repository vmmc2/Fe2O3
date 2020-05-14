//Functions P2...

//So far, we have only seen functions that don't end with an expression.
//Because Rust is an expression-based language, this is an important distinction to understand. 
//Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.

//Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.
//Em outras palavras, statements(declaracoes) nao retornam nada e nao fazem nenhum tipo de evaluation.
//Ja as expressions(expressoes), elas sim fazem uma evaluation e retornam um valor.

//Let's look at some examples..
//1) Creating a variable and assigning a value to it with the "let" keyword is a statement.
fn example1(){
    let mut x: i32 = 6;
}
//Function definitions are also statements. The entire preceding example is a statement in itself.

//2) The following code will result in an error because the use of the "let" keyword configures a statement.
fn mino(){
    //let x = (let y = 6); //The statement (let y = 6); does not return anything (unlike what happens in C/C++). So there isn't anything to bind to x.
}

//More about expressions:
//Expressions evaluate to something and make up most of the rest of the code that you’ll write in Rust. 
//Consider a simple math operation, such as 5 + 6, which is an expression that evaluates to the value 11. Expressions can be part of statements:
//Calling a function is an expression. Calling a macro is an expression. The block that we use to create new scopes, {}, is an expression, for example:

//This is another way of writing a function to return the n-th number in the fibonacci sequence: 1, 1, 2, 3, 5, 8, 13, 21, 33, 54, ...
//This is the more rustacean way.
fn fib(num: u64) -> u64{ //Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an arrow (->)
    if num == 1 || num == 2{
        1 //In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
    }else{
        fib(num - 1) + fib(num - 2) //In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
    }
    //You can return early from a function by using the "return" keyword and specifying a value, but most functions return the last expression implicitly.
}
fn main(){
    //let y = 13;
    let x = {
        let y = 6;
        y + 10
    };    
    
    println!("The value of fib(7) is: {}", fib(7));
}
//Expressions do not include ending semicolons. 
//If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. 
//Keep this in mind as you explore function return values and expressions next.


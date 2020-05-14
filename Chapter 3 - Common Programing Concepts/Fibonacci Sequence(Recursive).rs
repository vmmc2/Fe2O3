fn fibonacci(x: u32) -> u32{
    if x == 1 || x == 2{
        return 1;
    }else{
        return fibonacci(x - 1) + fibonacci(x - 2);
    }
}

fn main(){
    println!("Hello World!");
    
    println!("");
    println!("{}", fibonacci(10));
}

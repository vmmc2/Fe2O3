fn factorial(x: i64) -> i64{
    if x == 0{
        return 1;
    }else{
        return x * factorial(x - 1);
    }
}

fn main(){
    let num: i64 = 10;
    let answer: i64 = factorial(num);
    println!("The answer is: {}.", answer);
}

fn celsius_to_farenheit(temp: f32){
    let farenheit: f32 = ((9.0/5.0)*temp) + 32.0;
    println!("Temperature in Farenheit: {}", farenheit);
}
fn farenheit_to_celsius(temp: f32){
    let celsius: f32 = (5.0/9.0)*(temp - 32.0);
    println!("Temperature in Celsius: {}", celsius);
}

fn main(){
    let number: f32 = 36.5;
    celsius_to_farenheit(number);
    fp_division();
}

fn fp_division(){
    let result: f32 = (32 as f32)/6.0; //This is another way to convert from an integer number to a floating-point number (of 32-bits).
    println!("The value of the variable result is: {}", result);
}

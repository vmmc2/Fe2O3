//Rust is a statically-typed language. This means that it must know the type of all variables at compile-time
//The Rust Compiler, as well as the GHC, can usually infer what type we want to use based on its value and how we use it.

//Compound Types: Different from scalar types, compound types are types of data that can store/group multiple values inside just one type. Rust has two basic compound types: Arrays and Tuples.
//Em outras palavras, a gente tem varios dados que podem ser armazenados/guardados em um unico tipo.

//5) Arrays:
/*
- All elements inside an array must be of the same type. This means that if we have an array of integers, then we only have integers inside that array. If we have an array of floating-point numbers of 32-bits, then all the elements inside that array are floating-point numbers of size 32-bits. And it goes on for every type in Rust.
- Arrays in Rust are like arrays in C when it comes to size: They have a fixed size that must be specified when we are declaring the array. We cannot change that property.
- To access a specific element inside an array in Rust, we must use that index convention that comes from other languages like C/C++/Java.
- Arrays in Rust work 0-based indexes.
- Just as we saw in scalar types, all compound types are immutable by default. If we want to make them mutable we must use the "mut" keyword before the variable name.
- Arrays are really useful when we want our data to be allocated inside the Stack instead of the Heap. This is useful because we are dealing with data declared at compile-time.
- Unfortunately, the array data type is not as flexible as the vector type(which we are going to discuss later).
- If you are unsure whether you should use an array or a vector, then you probably should use a vector.
- Unlike Python, you cannot print the whole array in a single print statement. You must iterate through each element inside the array and then print it.
- You can also apply shadowing to array types. Just like we did when we were working with scalar types.
*/

fn main(){
    let vetor = [1, 2, 3, 4, 5, 6];
    println!("{}", vetor[0]);
    let vetor = ["dale", "porra"];
    println!("{}", vetor[0]);
    
    let mut array_mutavel = [1,2,3]; //Arrays em Rust podem ser mutaveis ou imutaveis. Depende da nossa necessidade.
    println!("The value of the first element inside the array is: {}", array_mutavel[0]);
    array_mutavel[0] = 69;
    println!("After making a change, the value of the first element inside this array is: {}", array_mutavel[0]);
    
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]; //Da para gente deixar imutavel porque a gente nunca vai mudar a qtd de dias que existem em uma semana certo??
    println!("{}", days[1]);
    
    //Existem outras maneiras de declararmos um array em Rust. A seguir, veremos como fazer isso identificando qual tipo de dado o array vai guardar, bem como o seu tamanho.
    //let nome_do_array: [tipo_de_dado; qtd_elementos] = [0, ..., qtd_elementos - 1];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let mut b: [i128; 3] = [13, 12, 11];
    
    //Tem outro jeito de declarar arrays tambem. A forma a seguir eh comumente utilizada quando queremos inicializar todos os elementos do nosso array para um certo valor
    //Seguindo a mesma linha da funcao memset() de C++.
    let mut c = [3; 5]; //Nesse caso, temos um array chamado c, que apresenta 5 elementos e todos eles apresetam o valor 3. Por definicao, esse valores sao do tipo i32.
    println!("The value of c[2] is: {}", c[2]);
    
    /*
    println!("The value of vetor[0] is: {}", vetor[0]);
    vetor[0] = 100;
    println!("The value of vetor[0] is: {}", vetor[0]);
    */
    
}

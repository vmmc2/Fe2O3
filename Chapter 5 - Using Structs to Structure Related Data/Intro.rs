//Structs sao utilizadas para agrupar dados que, em conjunto, apresentam um significado maior.
//Esses dados que estao presente em uma Structs sao chamados de campos/fields. Eles podem ser de qualquer outro tipo (assim como acontece com as tuplas)
//Structs sao flexiveis do que tuplas, pois nao temos que saber exatamente a ordem dos dados para poder altera-los ou acessa-los. Podemos fazer isso utilizando o nome dos campos

struct User{ //To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together.
    username: String, //A gente separa os campos/fields de uma Struct por ',' tanto na declaracao como na instanciacao.
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Processor{
    //Qual a estrutura do nosso Processor?
    registers: [i32; 32], //Um vetor que representa os 32 registradores. Temos 32 registradores, onde cada um armazena numeros com sinal de 32 bits
    pc: u64, //Um registrador especifico para o PC. Que armazena apenas numeros sem sinal (positivos) de 32 bits
    memory: [u8; 1000000], //Uma memoria indexada por bytes, que apresenta 1_000_000 de slots.
}

//To use a Struct after we have defined it inside our code, we must create an instance of this Struct. This is basically the same thing that happens
//when we are working with classes and objects in C++/Dart.
fn creating_user() -> User{
    //Again: To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields.
    //We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs, where the keys are the 
    //names of the fields and the values are the data we want to store in those fields. We don’t have to specify the fields in the same order in 
    //which we declared them in the struct.
    
    
    let user1: User = User{ //Da para realizar type annotations com Structs tambem.
        username: String::from("starlord000"),
        email: String::from("VictinhoMemes@email.com"),
        sign_in_count: 123,
        active: true,
    };
    return user1;
}

fn main(){
    let mut usuario = creating_user();
    println!("Nome de usuario: {}", usuario.username);
    usuario.username = String::from("Rei Leonidas");
    println!("Nome de usuario: {}", usuario.username);
    
    
    //Creating an instance of a Processor Stuct
    let mut cpu = Processor{
        registers: [0; 32], //Seto para 0 todos os registradores
        pc: 0, //Set o pc para 0
        memory: [1; 1000000], //Todos os slots de memoria comecarao com o valor 1.
    };
    println!("O valor do primeiro slot de memoria eh: {}", cpu.memory[0]);
}

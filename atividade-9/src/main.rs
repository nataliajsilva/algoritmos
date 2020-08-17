/*Efetuar a leitura de quatro valores numéricos inteiros, representados pelas variáveis A, B, C e D. Apresentar apenas os valores que sejam divisíveis por e 2 e 3. */

//Pseudocódigo
//Pedir ao usuário o número a
//Pedir ao usuário o número b
//Pedir ao usuário o número c
//Pedir ao usuário o número d
//Verificar quais desses números são divisivéis por 2 e 3
//Se algum número foi divisivél por 2 e 3, exibir na tela

use std :: io;

fn main() {
    
    println!("Verificando números divisivéis por 2 e 3");

    println!("Digite o número a");
    let mut numero_a : String = String::new();
    io::stdin()
       .read_line(&mut numero_a)
       .expect("Falha ao ler o valor");

    let numero_a : i32 = converter_string_para_i32(numero_a);



    println!("Digite o número b");
    let mut numero_b : String = String::new();
    io::stdin()
       .read_line(&mut numero_b)
       .expect("Falha ao ler o valor");

    let numero_b : i32 = converter_string_para_i32(numero_b);



    println!("Digite o número c");
    let mut numero_c : String = String::new();
    io::stdin()
       .read_line(&mut numero_c)
       .expect("Falha ao ler o valor");

    let numero_c : i32 = converter_string_para_i32(numero_c);

    
    
    println!("Digite o número d");
    let mut numero_d : String = String::new();
    io::stdin()
       .read_line(&mut numero_d)
       .expect("Falha ao ler o valor");

    let numero_d : i32 = converter_string_para_i32(numero_d);


    if numero_a % 2 == 0 && numero_a % 3 == 0 {
        println!("O número {} é divisivél por 2 e 3" , numero_a)
    }

    if numero_b % 2 == 0 && numero_b % 3 == 0 {
        println!("O número {} é divisivél por 2 e 3" , numero_b)
    }


    if numero_c % 2 == 0 && numero_c % 3 == 0 {
        println!("O número {} é divisivél por 2 e 3" , numero_c)
    }

    if numero_d % 2 == 0 && numero_d % 3 == 0 {
        println!("O número {} é divisivél por 2 e 3" , numero_d)
    }



    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}

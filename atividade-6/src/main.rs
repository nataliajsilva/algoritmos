/*Escrever um programa que receba três números inteiros como entrada e imprima, como saída, os números em ordem crescente.*/

//Pseudocódigo
//Pedir ao usuário o primeiro número inteiro
//Pedir ao usuário o segundo número inteiro
//Pedir ao usuário o terceiro número inteiro
//Colocar os números informados em ordem crescente
//Exibir os números em ordem crescente

use std::io;

fn main() {

    println!("ordenando números em ordem crescente");

    println!("Informe um número inteiro");

    let mut numero_a : String = String::new();
     io::stdin()
        .read_line(&mut numero_a)
        .expect("Falha ao ler o valor");

    let numero_a : i32 = converter_string_para_i32(numero_a);

    println!("Informe outro número inteiro");

    let mut numero_b : String = String::new();
     io::stdin()
        .read_line(&mut numero_b)
        .expect("Falha ao ler o valor");

    let numero_b : i32 = converter_string_para_i32(numero_b);

    println!("Informe mais um número inteiro");

    let mut numero_c : String = String::new();
     io::stdin()
        .read_line(&mut numero_c)
        .expect("Falha ao ler o valor");

    let numero_c : i32 = converter_string_para_i32(numero_c);


    if numero_a < numero_b {
        if numero_b < numero_c {
            println!("A ordem crescente é : {}, {}, {}", numero_a, numero_b, numero_c );
        } else if numero_a < numero_c {
            println!("A ordem crescente é : {}, {}, {}", numero_a, numero_c, numero_b );
        } else {
            println!("A ordem crescente é : {}, {}, {}", numero_c, numero_a, numero_b );
        }

    } else if numero_b < numero_c {
        if numero_a < numero_c {
            println!("A ordem crescente é : {}, {}, {}", numero_b, numero_a, numero_c );
        } else {
            println!("A ordem crescente é : {}, {}, {}", numero_b, numero_c, numero_a );
        }
    } else {
        println!("A ordem crescente é : {}, {}, {}", numero_c, numero_b, numero_a );
    }



    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
    
}

/*Construir um programa que leia um valor numérico inteiro e apresente como resultado armazenado os seus valores sucessor e antecessor.*/

//Pseudocódigo:
//Pedir ao usuário um valor númerico inteiro
//Realizar o cálculo do sucesso do número informado
//Realizar o cálculo do antecessor do número informado
//Apresentar o sucessor e o antecessor do número informado

use std::io;


fn main() {
    
    println!("Descobrindo o antecessor e o sucessor de um número inteiro");

    println!("Informe um número inteiro");

    let mut numero : String = String::new();
     io::stdin()
        .read_line(&mut numero)
        .expect("Falha ao ler o valor");

    let numero : i32 = converter_string_para_i32(numero);

    let antecessor = numero - 1;
    let sucessor = numero + 1;

    println!("O antecessor do número {} é o {}, e o sucessor é o {}", numero, antecessor, sucessor);

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
    
}

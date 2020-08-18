/*Efetuar a leitura de um valor numérico inteiro que esteja na faixa de 1 até 9. O programa deverá apresentar uma mensagem "O valor está na faixa permitida", caso o valor informado esteja na faixa. Se o valor estiver fora da faixa permitida, deverá apresentar a mensagem "O valor está fora da faixa permitida". */

//Pseudocódigo:
//Pedir ao uusário um número inteiro que esteja entre 1 e 9
//Verificar se o número informado esta entre 1 e 9
//Caso esteja exibir a msg que o valor esta entre a faixa permitida, caso contrário informar que esta fora da faixa

use std :: io;

fn main() {

    println!("Números inteiros entre 1 e 9");

    println!("Digite um número que esteja entre 1 e 9");
    let mut numero : String = String::new();
    io::stdin()
       .read_line(&mut numero)
       .expect("Falha ao ler o valor");

    let numero : i32 = converter_string_para_i32(numero);

    if numero >= 1 && numero <= 9 {
        println!("O valor está na faixa permitida");

    } else {
        println!("O valor está fora da faixa permitida");
    }
    

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
    
}

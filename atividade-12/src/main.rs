/*Efetuar a leitura de três valores inteiros desconhecidos representados pelas variáveis A, B e C. Somar os valores fornecidos e apresentar o resultado somente se for maior ou igual a 100.*/

//Pseudocódigo:
//Pedir ao usuário o valor do número a
//Pedir ao usuário o valor do número b
//Pedir ao usuário o valor do número c
//Realizar a soma dos valores fornecidos
//Se o resultado for maior do que 100, apresentar o valor na tela, caso contrário não exibir


use std :: io;

fn main() {
    
    println!("Somando 3 números inteiros que somados dão 100");

    println!("Digite o valor do número A");
    let mut numero_a : String = String::new();
    io::stdin()
       .read_line(&mut numero_a)
       .expect("Falha ao ler o valor");

    let numero_a : i32 = converter_string_para_i32(numero_a);

    println!("Digite o valor do número B");
    let mut numero_b : String = String::new();
    io::stdin()
       .read_line(&mut numero_b)
       .expect("Falha ao ler o valor");

    let numero_b : i32 = converter_string_para_i32(numero_b);

    println!("Digite o valor do número C");
    let mut numero_c : String = String::new();
    io::stdin()
       .read_line(&mut numero_c)
       .expect("Falha ao ler o valor");

    let numero_c : i32 = converter_string_para_i32(numero_c);

    let soma = numero_a + numero_b + numero_c;

    if soma >= 100 {
        println!("Somando os 3 números informados o resultado é maior ou igual a 100. Operação realizada: {} + {} + {} = {}" 
        ,numero_a , numero_b, numero_c, soma);

    }else {
        println!("A soma dos 3 números não resulta no valor 100");
    }

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }


}

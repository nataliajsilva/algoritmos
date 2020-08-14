/*Elaborar um programa que leia dois valores numéricos inteiros, os quais devem representar a base e o expoente de uma potência. Calcular a potência, e exibir a operação calculada, e o resultado obtido. */

//Pseudocódigo:
//Pedir ao usuário um valor númerico inteiro que representará a base de uma potência
//Pedir ao usuário outro valor númerico inteiro que representará o expoente de uma potência
//Calcular a potência
//Apresentar o resultado obtido

use std :: io;

fn main() {

println!("Calculando Potênciação");

println!("Informe um número inteiro que será a base da potência");
let mut base : String = String::new();
io::stdin()
        .read_line(&mut base)
        .expect("Falha ao ler o valor");

let base : i32 = converter_string_para_i32(base);



println!("Informe um número inteiro que será o exponente da potência");
let mut expoente : String = String::new();
io::stdin()
        .read_line(&mut expoente)
        .expect("Falha ao ler o valor");

let expoente : u32 = converter_string_para_u32(expoente);



let pontenciacao = base.pow(expoente);


println!("O resultado da potênciação {}^{} é : {}", base, expoente, pontenciacao);


fn converter_string_para_i32(uma_string: String) -> i32 {
    uma_string
        .trim()
        .parse()
        .expect("Falha ao converter o valor")
}

fn converter_string_para_u32(uma_string: String) -> u32 {
    uma_string
        .trim()
        .parse()
        .expect("Falha ao converter o valor")
}
    
}

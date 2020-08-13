/* Elaborar um programa que calcule e apresente o valor do volume de uma caixa retangular,
utilizando a fórmula:
Volume ← Comprimento * Largura * Altura 
*/


//Pseudocódigo:
//Pedir ao usuário o valor de Comprimento da caixa
//Pedir ao usuário o valor de Largura da caixa
//Pedir ao usuário o valavor de Altura da caixa

//Realizar o calculo do volume: Volume ← Comprimento * Largura * Altura

//E exibir ao usuário o resultado do volume

use std::io;

fn main () {

    println!("Calculando o volume de uma caixa retangular.");

    println!("Digite o valor de comprimento da caixa.");
    let mut comprimento : String = String::new();
    io::stdin()
        .read_line(&mut comprimento)
        .expect("Falha ao ler o valor");

    
    let comprimento : i32 = converter_string_para_i32(comprimento);


    println!("Digite o valor de largura da caixa.");
    let mut largura : String = String::new();
    io::stdin()
        .read_line(&mut largura)
        .expect("Falha ao ler o valor");

    let largura : i32 = converter_string_para_i32(largura);


    println!("Digite o valor de altura da caixa.");
    let mut altura : String = String::new();
    io::stdin()
        .read_line(&mut altura)
        .expect("Falha ao ler o valor");
    
    let altura : i32 = converter_string_para_i32(altura);


    let volume = comprimento * largura * altura;
    println!("O volume da caixa é: {} m³." ,volume);


    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }


}



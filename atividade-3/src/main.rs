/*Elaborar um programa que leia dois valores números reais representados pelas variáveis A e B. Calcular, armazenar e apresentar os resultados das quatro operações aritméticas básicas entre eles.*/

//Pseudocódigo:
//Pedir ao usuário o valor do número A
//Pedor ao usuário o valor do número B
//Realizar o cálculo das quatro operações aritméticas básicas (/ , * , + ,  - ) utilizando os valores do das variáveis A e B, e armazenar os resultados
//Apresentar os resultados das operações entre os valores

use std::io;

fn main() {

println!("Cálculo das quatro operações aritméticas básicas com dois valores");

println!("Informe o valor do número A");
let mut a : String = String::new();
io::stdin()
        .read_line(&mut a)
        .expect("Falha ao ler o valor");

let a : f32 = converter_string_para_f32(a);



println!("Informe o valor do número B");
let mut b : String = String::new();
io::stdin()
        .read_line(&mut b)
        .expect("Falha ao ler o valor");

let b : f32 = converter_string_para_f32(b);



let soma_divisao_ab = a/b;
let soma_divisao_ba = b/a;
let soma_multiplicacao_ab = a*b;
let soma_multiplicacao_ba = b*a;
let soma_adicao_ab = a+b;
let soma_adicao_ba = b+a;
let soma_subtracao_ab = a-b;
let soma_subtracao_ba = b-a;



println!("O resultado das quatro operações aritméticas utilizando os valores de A e B são: ");
println!("Divisão A/B e B/A: {} , {}", soma_divisao_ab, soma_divisao_ba);
println!("Multiplicação A*B e B*A: {} , {}", soma_multiplicacao_ab, soma_multiplicacao_ba);
println!("Adição A+B e B+A: {} , {}", soma_adicao_ab, soma_adicao_ba);
println!("Subtração A-B e B-A: {} , {}", soma_subtracao_ab, soma_subtracao_ba);



fn converter_string_para_f32(uma_string: String) -> f32 {
            uma_string
                .trim()
                .parse()
                .expect("Falha ao converter o valor")
        }
    
}

/*Elaborar um programa que apresente o valor da conversão em dólar (US$) de um valor lido em real (R$). O programa deve solicitar o valor da cotação do dólar e também a quantidade de de reais disponível com o usuário e armazenar em variáveis o valor da conversão antes da apresentação.*/

//Pseudocódigo:
//Pedir ao usuário o valor da cotação do dólar
//Pedir ao usuário a quantidade de reais que será feita a conversão
//Realizar cálculo da conversão
//Apresentar o valor da conversão


use std::io;

fn main() {

println!("Conversão moedas");

println!("Informe o valor da cotação do dólar (US$)");
let mut dolar : String= String::new();
io::stdin()
        .read_line(&mut dolar)
        .expect("Falha ao ler o valor");

let dolar : f32 = converter_string_para_f32(dolar);


println!("Se deseja realizar a conversão de real para dólar digite 1, caso contrário será feita a conversão de dólar para real ");

let mut selecao_conversao_dolar : String = String::new();
io::stdin()
        .read_line(&mut selecao_conversao_dolar)
        .expect("Falha ao ler o valor");

let selecao_conversao_dolar : i32 = converter_string_para_i32(selecao_conversao_dolar);

if selecao_conversao_dolar == 1{

    println!("Informe a quantia em reais (R$) que deseja realizar a conversão em dólar");
    let mut qtd_real : String= String::new();
    io::stdin()
            .read_line(&mut qtd_real)
            .expect("Falha ao ler o valor");
    
    let qtd_real : f32 = converter_string_para_f32(qtd_real);
    
    
    let conversao_dolar = qtd_real/dolar;
    
    println!("A conversão de R$ {} é equivalente a US$ {}", qtd_real, conversao_dolar);

} else {
    println!("Informe a quantia em dólar (US$) que deseja realizar a conversão em real");
    let mut qtd_dolar : String= String::new();
    io::stdin()
            .read_line(&mut qtd_dolar)
            .expect("Falha ao ler o valor");
    
    let qtd_dolar : f32 = converter_string_para_f32(qtd_dolar);
    
    
    let conversao_real = qtd_dolar*dolar;
    
    println!("A conversão de US$ {} é equivalente a R$ {}", qtd_dolar, conversao_real);

}


fn converter_string_para_f32(uma_string: String) -> f32 {
            uma_string
                .trim()
                .parse()
                .expect("Falha ao converter o valor")
        }

fn converter_string_para_i32(uma_string: String) -> i32 {
            uma_string
                .trim()
                .parse()
                .expect("Falha ao converter o valor")
        }


}
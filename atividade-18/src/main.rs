/*Elaborar um programa que pergunte quantos números inteiros deverão ser lidos, lê-los, e no final apresente o somatório da fatorial de todos os valores lidos. */

use std::io;

fn main() {
    println!("Somatório da fatorial de todos os números lidos");

    let mut numeros: [i32; 3] = [0; 3];
    let mut numeros_indice: usize = 0;

    while numeros_indice <= 2 {
        println!("Digite um número");
        let mut numero: String = String::new();
        io::stdin()
            .read_line(&mut numero)
            .expect("Falha ao ler o valor");
        let numero: i32 = converter_string_para_i32(numero);

        numeros[numeros_indice] = numero;
        numeros_indice = numeros_indice + 1;
    }

    let mut soma_fatorial: i32 = 0;

    for i in numeros.iter() {

        let mut contador: i32 = 1;
        let mut resultado_fatorial: i32 = 1;

        while contador <= *i {

            resultado_fatorial = resultado_fatorial * contador;
            contador = contador + 1;

        //println!("{}", resultado_fatorial);
        }
        soma_fatorial += resultado_fatorial;   
    }

    println!("O somatório das fatorias dos números é {} ", soma_fatorial);

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}

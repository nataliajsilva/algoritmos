/*Elaborar um programa que pergunte quantos números inteiros deverão ser lidos, lê-los, e no final apresente o somatório da fatorial de todos os valores lidos. */

use std::io;

fn main() {
    println!("Somatório da fatorial de todos os números lidos");

    println!("Quantos números inteiros deverão ser lidos?");
    let mut qtd_numero: String = String::new();
    io::stdin()
        .read_line(&mut qtd_numero)
        .expect("Falha ao ler o valor");

    let qtd_numero: i32 = converter_string_para_i32(qtd_numero);
    let mut numeros = vec![];
    let mut contador: i32 = 1;

    while contador <= qtd_numero {
        println!("Digite o valor do número");
        let mut numero: String = String::new();
        io::stdin()
            .read_line(&mut numero)
            .expect("Falha ao ler o valor");
        let numero: i32 = converter_string_para_i32(numero);

        numeros.push(numero);
        contador = contador + 1;
    }

    //println!("{}", numeros[0]);

    for i in numeros.iter() {
        let mut fatorial: usize;
        fatorial = fatorial * (numeros & [i] - 1, 0usize);
        numeros & [i] - 1 as usize;

        println!("{}", fatorial);
    }

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}

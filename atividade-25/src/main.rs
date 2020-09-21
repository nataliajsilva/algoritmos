/*Elaborar um programa que leia um vetor A com dez elementos numéricos inteiros. Apresentar o total de elementos ímpares existente no vetor e também o percentual do valor total de números ímpares em relação à quantidade total de elementos armazenados.*/

use std::io;

fn main() {
    println!("Exibindo valores ímpares do vetor A, e a porcentagem que representa");

    let mut a: [i32; 10] = [0; 10];
    let mut numeros_indice_a: usize = 0;

    while numeros_indice_a <= 9 {
        println!("Digite um número inteiro para inserir no vetor A");
        let mut numeros: String = String::new();
        io::stdin()
            .read_line(&mut numeros)
            .expect("Falha ao ler o valor");
        let numeros: i32 = converter_string_para_i32(numeros);
    
        a[numeros_indice_a] = numeros;
        numeros_indice_a = numeros_indice_a + 1;
    }

    println!("{:?},",a);

    let mut total_impares : i32 = 0;

    for i in a.iter() {

            if i % 2 == 0 {
                
            } else {
                total_impares = total_impares + 1;
            }    
    }

    println!("O total de números ímpares no vetor A é de :{}" , total_impares);

    let porcentagem_impares = total_impares * 100 / 10 ;

    println!("O percentual do valor total de números ímpares em relação à quantidade total de elementos armazenados no vetor A é de :{}%" , porcentagem_impares);

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
         }
}

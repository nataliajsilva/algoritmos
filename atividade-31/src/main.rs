/*Elaborar um programa que leia uma matriz A do tipo inteiro de cinco colunas e cinco linhas. Ao final, apresentar o total de elementos pares e ímpares existentes na matriz. Apresentar também o percentual de elementos pares e ímpares em relação ao total de elementos.*/

use std::io;

fn main() {
    println!("Exibindo o total de elementos pares e ímpares, e o percentual com relação ao total de elementos existentes na matriz A .");

    let colunas_matriz_a = 5;
    let linhas_matriz_a = 5;

    let mut a = vec![vec![0; colunas_matriz_a]; linhas_matriz_a];
    for x in 0..linhas_matriz_a {
        for y in 0..colunas_matriz_a {
            println!("Digite um número inteiro para inserir na matriz A");

            let mut numeros_matriz_a: String = String::new();
            io::stdin()
                .read_line(&mut numeros_matriz_a)
                .expect("Falha ao ler o valor");
            let numeros_matriz_a: i32 = converter_string_para_i32(numeros_matriz_a);

            a[x][y] = numeros_matriz_a;
        }
    }

    let colunas = 5;
    let linhas = 5;
    let mut total_pares = 0;
    let mut total_impares = 0;

    for x in 0..colunas {
        for y in 0..linhas {
            if a[x][y] % 2 == 0 {
                total_pares = total_pares + 1;
            } else {
                total_impares = total_impares + 1;
            }
        }
    }

    let percentual_pares = total_pares * 100 / (5 * 5);
    let percentual_impares = total_impares * 100 / (5 * 5);

    println!("A={:?}", a);
    println!(
        "O total de elementos pares existentes na matriz A é: {} ",
        total_pares
    );

    println!(
        "O total de elementos impares existentes na matriz A é: {}",
        total_impares
    );
    println!("O percentual de elementos pares em relação ao total de elementos existentes na matriz A é: {}%",percentual_pares);
    println!("O percentual de elementos impares em relação ao total de elementos existentes na matriz A é: {}%",percentual_impares);

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}

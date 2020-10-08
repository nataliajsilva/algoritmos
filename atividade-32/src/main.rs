/*Elaborar um programa que leia uma matriz A e B do tipo inteiro de cinco colunas e cinco linhas. A matriz A deve ser formada por valores divisíveis por 3 e 4, enquanto a matriz B deve ser formada por valores divisíveis por 5 e 6. As entradas do usuário deverão ser validadas pelo programa. Construir e apresentar uma matriz C de mesma dimensão que contenha o valor da multiplicação dos elementos da matriz A com os elementos correspondentes da matriz B. Apresentar as três matrizes.*/

use std::io;

fn main() {
    let colunas_matriz_a = 5;
    let linhas_matriz_a = 5;

    let mut a = vec![vec![0; colunas_matriz_a]; linhas_matriz_a];
    for x in 0..linhas_matriz_a {
        for y in 0..colunas_matriz_a {
            println!("Digite um número que seja divisível por 3 ou 4 para inserir na matriz A");

            let mut numeros_matriz_a: String = String::new();
            io::stdin()
                .read_line(&mut numeros_matriz_a)
                .expect("Falha ao ler o valor");
            let numeros_matriz_a: i32 = converter_string_para_i32(numeros_matriz_a);

            if numeros_matriz_a % 3 == 0 || numeros_matriz_a % 4 == 0 {
                a[x][y] = numeros_matriz_a;
            } else {
                println!("Número informado não é divisível por 3 ou 4, por favor informe outro");
            }
        }
    }

    let colunas_matriz_b = 5;
    let linhas_matriz_b = 5;

    let mut b = vec![vec![0; colunas_matriz_b]; linhas_matriz_b];
    for x in 0..linhas_matriz_b {
        for y in 0..colunas_matriz_b {
            println!(
                "Digite um número inteiro que seja divisível por 5 ou 6 para inserir na matriz B"
            );

            let mut numeros_matriz_b: String = String::new();
            io::stdin()
                .read_line(&mut numeros_matriz_b)
                .expect("Falha ao ler o valor");
            let numeros_matriz_b: i32 = converter_string_para_i32(numeros_matriz_b);

            if numeros_matriz_b % 5 == 0 || numeros_matriz_b % 6 == 0 {
                b[x][y] = numeros_matriz_b;
            } else {
                println!("Número informado não é divisível por 5 ou 6, por favor informe outro");
            }
        }
    }

    let mut c = vec![vec![0; 5]; 5];

    let colunas = 5;
    let linhas = 5;

    for x in 0..colunas {
        for y in 0..linhas {
            c[x][y] = a[x][y] * b[x][y];
        }
    }

    println!("A={:?}", a);
    println!("B={:?}", b);
    println!("C={:?}", c);

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}

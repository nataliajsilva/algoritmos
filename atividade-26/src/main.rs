/*Elaborar um programa que leia duas matrizes A e B, cada uma de duas dimensões com três linhas e três colunas para valores inteiros. Construir uma matriz C de mesma dimensão, que seja formada da soma dos elementos da matriz A com os elementos da matriz B. Apresentar os elementos da matriz C.*/

use std::io;

fn main() {
    println!("Exibindo matriz C, criada a partir da soma dos elementos das matrizes A e B");

    let colunas_matriz_a = 3;
    let linhas_matriz_a = 3;

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

    let colunas_matriz_b = 3;
    let linhas_matriz_b = 3;

    let mut b = vec![vec![0; colunas_matriz_b]; linhas_matriz_b];

    for x in 0..linhas_matriz_b {
        for y in 0..colunas_matriz_b {
            println!("Digite um número inteiro para inserir na matriz B");

            let mut numeros_matriz_b: String = String::new();
            io::stdin()
                .read_line(&mut numeros_matriz_b)
                .expect("Falha ao ler o valor");
            let numeros_matriz_b: i32 = converter_string_para_i32(numeros_matriz_b);

            b[x][y] = numeros_matriz_b;
        }
    }

    let mut c = vec![vec![0; 3]; 3];

    let colunas = 3;
    let linhas = 3;

    for x in 0..colunas {
        for y in 0..linhas {
            c[x][y] = a[x][y] + b[x][y];
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

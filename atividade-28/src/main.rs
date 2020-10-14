/*Elaborar um programa que leia uma matriz A 5x5. Construir uma matriz B 5x5 em que cada elemento seja o dobro correspondente à matriz A, com exceção dos elementos da diagonal principal, os quais devem ser o triplo de cada elemento correspondente da matriz A. Apresentar a matriz B.*/

use std::io;

fn main() {
    println!("Exibindo matriz B, criada a partir do dobro correspondente à matriz A, com exceção dos elementos da diagonal principal, os quais devem ser o triplo de cada elemento correspondente da matriz A");

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

    let mut b = vec![vec![0; 5]; 5];

    let colunas = 5;
    let linhas = 5;
    let mut indice_diagonal = 0;

    for x in 0..colunas {
        for y in 0..linhas {
            if x == indice_diagonal && y == indice_diagonal {
                b[x][y] = a[x][y] * 3;
            } else {
                b[x][y] = a[x][y] * 2;
            }
        }
        indice_diagonal += 1;
    }

    println!("A={:?}", a);
    println!("B={:?}", b);

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}

/*Elaborar um programa que leia uma matriz A do tipo real de duas dimensões com cinco linhas e cinco colunas. Apresentar o somatório dos elementos situados na diagonal principal.*/

use std::io;

fn main() {
    println!("Exibindo o somatório dos elementos situados na diagonal principal da matriz A.");

    let colunas_matriz_a = 5;
    let linhas_matriz_a = 5;

    let mut a = vec![vec![0.0; colunas_matriz_a]; linhas_matriz_a];
    for x in 0..linhas_matriz_a {
        for y in 0..colunas_matriz_a {
            println!("Digite um número real para inserir na matriz A");

            let mut numeros_matriz_a: String = String::new();
            io::stdin()
                .read_line(&mut numeros_matriz_a)
                .expect("Falha ao ler o valor");
            let numeros_matriz_a: f32 = converter_string_para_f32(numeros_matriz_a);

            a[x][y] = numeros_matriz_a;
        }
    }

    let colunas = 5;
    let linhas = 5;
    let mut somatorio = 0.0;

    for x in 0..colunas {
        for y in 0..linhas {
            if x == 0 && y == 0
                || x == 1 && y == 1
                || x == 2 && y == 2
                || x == 3 && y == 3
                || x == 4 && y == 4
            {
                somatorio = somatorio + a[x][y];
            }
        }
    }

    println!("A={:?}", a);
    println!(
        "O somatório dos elementos situados na diagonal principal da matriz A é : {} ",
        somatorio
    );

    fn converter_string_para_f32(uma_string: String) -> f32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}

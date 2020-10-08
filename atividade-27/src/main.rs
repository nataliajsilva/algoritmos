/*Elaborar um programa que leia dois vetores A e B, para 5 elementos reais. Construir uma matriz C de duas dimensões, sendo a primeira coluna da matriz C formada pelos elementos do vetor A multiplicados por 2 e a segunda coluna formada pelos elementos do vetor B subtraídos por 5. Apresentar a matriz C.*/

use std::io;

fn main() {
    println!("Exibindo a matriz C a partir da multiplicação dos elementos do vetor A e subtração dos elementos do vetor B");

    let mut a: [f32; 5] = [0.0; 5];
    let mut numeros_indice_a: usize = 0;

    while numeros_indice_a <= 4 {
        println!("Digite um número para inserir no vetor A");
        let mut numeros_a: String = String::new();
        io::stdin()
            .read_line(&mut numeros_a)
            .expect("Falha ao ler o valor");
        let numeros_a: f32 = converter_string_para_f32(numeros_a);
        a[numeros_indice_a] = numeros_a;
        numeros_indice_a = numeros_indice_a + 1;
    }

    let mut b: [f32; 5] = [0.0; 5];
    let mut numeros_indice_b: usize = 0;

    while numeros_indice_b <= 4 {
        println!("Digite um número para inserir no vetor B");
        let mut numeros_b: String = String::new();
        io::stdin()
            .read_line(&mut numeros_b)
            .expect("Falha ao ler o valor");
        let numeros_b: f32 = converter_string_para_f32(numeros_b);
        b[numeros_indice_b] = numeros_b;
        numeros_indice_b = numeros_indice_b + 1;
    }

    let mut c = vec![vec![0.0; 2]; 5];

    let coluna_um: usize = 0;
    let coluna_dois: usize = 1;
    let mut contador: usize = 0;
    let mut contador_linhas: usize = 0;

    while contador_linhas <= 4 {
        while contador <= 4 {
            c[contador_linhas][coluna_um] = a[contador] * 2.0;
            c[contador_linhas][coluna_dois] = b[contador] - 5.0;
            contador = contador + 1;
            contador_linhas = contador_linhas + 1;
        }
    }

    println!("A={:?}", a);
    println!("B={:?}", b);
    println!("C={:?}", c);

    fn converter_string_para_f32(uma_string: String) -> f32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}

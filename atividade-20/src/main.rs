/*Elaborar um programa que leia dois vetores (denominados A e B) com 5 elementos reais. Construir um vetor denominado C, onde cada elemento deverá corresponder a subtração do elemento correspondente do vetor A com um elemento correspondente do vetor B. Ao final, apresentar os elementos do vetor C.*/

use std::io; 

fn main() {
    
    println!("Exibindo novo vetor a partir da subtração de dois outros vetores");

    let mut a: [i32; 5] = [0; 5];
    let mut numeros_indice_a: usize = 0;

    while numeros_indice_a <= 4 {
        println!("Digite um número para o vetor A");
        let mut numeros_a: String = String::new();
        io::stdin()
            .read_line(&mut numeros_a)
            .expect("Falha ao ler o valor");
        let numeros_a: i32 = converter_string_para_i32(numeros_a);
    
        a[numeros_indice_a] = numeros_a;
        numeros_indice_a = numeros_indice_a + 1;
    }

    let mut b: [i32; 5] = [0; 5];
    let mut numeros_indice_b: usize = 0;

    while numeros_indice_b <= 4 {
        println!("Digite um número para o vetor B");
        let mut numeros_b: String = String::new();
        io::stdin()
            .read_line(&mut numeros_b)
            .expect("Falha ao ler o valor");
        let numeros_b: i32 = converter_string_para_i32(numeros_b);
    
        b[numeros_indice_b] = numeros_b;
        numeros_indice_b = numeros_indice_b + 1;
    }

    println!("A={:?}", a);
    println!("B={:?}", b);


    let mut c: [i32; 5] = [0; 5];

    let mut contador: usize = 0;

    while contador <= 4 {

        c[contador] = a[contador] - b[contador]; 
        contador = contador + 1;

    }

    println!("C={:?}", c);
    
    fn converter_string_para_i32(uma_string: String) -> i32 {
       uma_string
          .trim()
          .parse()
          .expect("Falha ao converter o valor")
    }
}

/*Elaborar um programa que leia um vetor A de 5 elementos inteiros. Construir um vetor B do mesmo tipo, e cada elemento do vetor B deve ser o resultado da fatorial correspondente de cada elemento do vetor A. Apresentar os vetores A e B.*/

use std::io;

fn main() {

    println!("Construindo o vetor B a partir da fatorial de cada elemento do vetor A");

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
    let mut resultado_fatorial: i32 = 1;

    for i in a.iter() {

        let mut contador: i32 = 1;

        while contador <= *i {

            resultado_fatorial = resultado_fatorial * contador;
            contador = contador + 1;

            b[contador]= resultado_fatorial;

        println!("{}", resultado_fatorial);
        }

        resultado_fatorial = 1;
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

/*Elaborar um programa que leia dois vetores A e B com 5 elementos. O vetor A deve aceitar apenas a entrada de valores divisíveis por 2 ou 3, enquanto o vetor B deve aceitar apenas a entrada de valores que não sejam múltiplos de 5. A entrada dos vetores deve ser validada pelo programa, e não pelo usuário. Construir um vetor C que seja o resultado da junção dos vetores A e B, de forma que contenha 10 elementos. Apresentar os elementos do vetor C.*/


use std::io;

fn main() {
    
    println!("Exibindo vetor com valores divisíveis por 2 ou 3, e multiplos de 5");

    let mut a: [i32; 5] = [0; 5];
    let mut numeros_indice_a: usize = 0;

    while numeros_indice_a <= 4 {
        println!("Digite um número que seja divisívell por 2 ou 3 no vetor A");
        let mut numeros: String = String::new();
        io::stdin()
            .read_line(&mut numeros)
            .expect("Falha ao ler o valor");
        let numeros: i32 = converter_string_para_i32(numeros);

        if numeros % 2 == 0 || numeros % 3 == 0 {

           a[numeros_indice_a] = numeros;
           numeros_indice_a = numeros_indice_a + 1;

        } else {
            println!("Número informado não é divisível por 2 ou 3, por favor informe outro");
        }
    }

    let mut b: [i32; 5] = [0; 5];
    let mut numeros_indice_b: usize = 0;

    while numeros_indice_b <= 4 {
        println!("Digite um número que não seja múlltiplo de 5 no vetor B");
        let mut numeros: String = String::new();
        io::stdin()
            .read_line(&mut numeros)
            .expect("Falha ao ler o valor");
        let numeros: i32 = converter_string_para_i32(numeros);

        if  numeros % 5 != 0 {

           b[numeros_indice_b] = numeros;
           numeros_indice_b = numeros_indice_b + 1;

        } else {
            println!("Número informado é múltiplo de 5, por favor informe outro");
        }
    }

    println!("A= {:?},",a);
    println!("B= {:?},",b);

    let mut c: [i32; 10] = [0; 10];
    
    let mut contador: usize = 0;
    let mut contador_b : usize = 0;

    while contador <= 9 {

        if contador <= 4 {
            c[contador] = a[contador];
        } else {
            c[contador] = b[contador_b];
            contador_b += 1;
        }
       contador += 1;
    }

    println!("C= {:?},",c);

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
         }
}

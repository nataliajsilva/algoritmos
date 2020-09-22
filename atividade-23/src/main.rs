/*Elaborar um programa que leia 10 elementos inteiros e armazene em um vetor A. Apresentar os valores do vetor lido de maneira inversa, e identificar qual é o menor valor armazenado.*/


use std::io;

fn main() {
   
    println!("Exibindo o vetor A inverso, e o menor número lido");

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

    let mut menor_numero :i32 = a[0];
    let mut vetor_inverso :[i32; 10] = [0; 10];
    
    let mut contador : usize = 9;
    let mut contador_inverso : usize = 0;

    while contador >=  0 {

        vetor_inverso[contador_inverso] = a[contador];
        
        if contador > 0 {
            contador -= 1; 
            contador_inverso += 1;

            if a[contador] < menor_numero {
                menor_numero = a[contador];
            } 
        } else {
            break;
        }
    }
    
    println!("A={:?}", a);
    println!("Vetor inverso={:?}", vetor_inverso);
    println!("O menor número lido no vetor foi {}", menor_numero);
        
    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
         }
    } 
         
   



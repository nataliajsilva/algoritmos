/*Elaborar um programa para calcular o máximo divisor comum de dois números inteiros positivos, MDC(x, y), usando o algoritmo de Euclides. Este algoritmo é baseado no fato de que se o resto da divisão de x por y representado por r, for igual a zero, y é o MDC. Se o resto r for diferente de zero, o MDC de x e y é igual ao MDC de y e r. O processo se repete até que o valor do resto da divisão seja zero.*/

use std::io;

fn main() {
    println!("Calculando o MDC de dois números inteiros positivos");

    let mut mdc_numeros: [i32; 2] = [0; 2];
    let mut numeros_indice: usize = 0;

    while numeros_indice <= 1 {
        println!("Digite um número");
        let mut numero: String = String::new();
        io::stdin()
            .read_line(&mut numero)
            .expect("Falha ao ler o valor");
        let numero: i32 = converter_string_para_i32(numero);

        mdc_numeros[numeros_indice] = numero;
        numeros_indice = numeros_indice + 1;
    }
    
    fn calculando_mdc(mut a : i32, mut b : i32) -> i32{
        let mut resto;
        
        while b != 0 {
        resto = a % b;
        a = b;
        b = resto; 
    }
     return a;
  }

  let x = mdc_numeros[0];
  let y = mdc_numeros[1];

  println!("O MDC de {} e {} é {}", x, y,calculando_mdc(x,y));

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}


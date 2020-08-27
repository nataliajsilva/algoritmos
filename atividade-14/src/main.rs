/*Elaborar um programa que mostre os resultados da tabuada, de 0 até 10, de um número qualquer.

Restrição: Utilize o laço condicional pré-teste. */

//Pseudocódigo
//Pedir ao usuário qual número deseja ver a tabuada
//Construir condições que correspondam ao resultado das tabuadas de 0 a 10 e verificar se o número escolhido pelo usuário possui uma condição correspondente
//Exibir para o usuário o resultado da tabuada

use std::io;

fn main() {

    println!("Tabuada de 0 até 10");

    println!("Digite um número de 0 a 10, que deseje ver a tabuada");
    let mut numero : String = String::new();
    io::stdin()
       .read_line(&mut numero)
       .expect("Falha ao ler o valor");

    let numero : i32 = converter_string_para_i32(numero);
    let mut contador : i32 = 0;

    while numero == 0 && contador <= 10 {
        println!(" {} x {} = {}" , numero, contador, numero * contador);
        contador = contador + 1;
        }
        
    while numero == 1 && contador <= 10 {
        println!(" {} x {} = {}" , numero, contador, numero * contador);
        contador = contador + 1;
        }

    while numero == 2 && contador <= 10 {
        println!(" {} x {} = {}" , numero, contador, numero * contador);
        contador = contador + 1;
        }

    while numero == 3 && contador <= 10 {
        println!(" {} x {} = {}" , numero, contador, numero * contador);
        contador = contador + 1;
        }

    while numero == 4 && contador <= 10 {
    println!(" {} x {} = {}" , numero, contador, numero * contador);
    contador = contador + 1;
    }

    while numero == 5 && contador <= 10 {
        println!(" {} x {} = {}" , numero, contador, numero * contador);
        contador = contador + 1;
        }
    
    while numero == 6 && contador <= 10 {
        println!(" {} x {} = {}" , numero, contador, numero * contador);
        contador = contador + 1;
        }
    
    while numero == 7 && contador <= 10 {
        println!(" {} x {} = {}" , numero, contador, numero * contador);
        contador = contador + 1;
        }

    while numero == 8 && contador <= 10 {
        println!(" {} x {} = {}" , numero, contador, numero * contador);
        contador = contador + 1;
        }
    
    while numero == 9 && contador <= 10 {
        println!(" {} x {} = {}" , numero, contador, numero * contador);
        contador = contador + 1;
        }

    while numero == 10 && contador <= 10 {
        println!(" {} x {} = {}" , numero, contador, numero * contador);
        contador = contador + 1;
        }

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}

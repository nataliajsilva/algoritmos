/*Ler um valor numérico inteiro e apresente uma mensagem informando se o valor fornecido é par ou ímpar. */

//Pseudocódigo
//Pedir ao usuário um número
//Verificar se o número informado é impar ou par
//Exibir o resultado

use std :: io;

fn main() {

    println!("Verificando se um número é impar ou par");

    println!("Digite um número inteiro");
    let mut numero : String = String::new();
    io::stdin()
       .read_line(&mut numero)
       .expect("Falha ao ler o valor");

    let numero : i32 = converter_string_para_i32(numero);
    

    if numero % 2 == 0 {
        println!("O número {} é par" , numero);
    } else {
        println!("O número {} é impar" , numero);
    }


    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
    
}

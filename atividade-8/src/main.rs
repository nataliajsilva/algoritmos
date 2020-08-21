
/*A sequência de Fibonacci é formada pelos números:
0, 1, 2, 3, 5, 8, 13, 21, … 
Sendo o primeiro número da série 0, o segundo 1, e os subsequentes a soma dos dois anteriores. Escrever um programa que imprima os 13 primeiros termos da série. */

//Pseudocódigo
//Realizar a lógica da sequência que escreva 13 números
//Exibir os 13 primeiros números da sequência

fn main() {

    /*let quantidade_numeros : i32 = 13;
    let mut contador : i32 = 0;
    let mut numero_anterior : i32 = -1;
    let mut numero_atual : i32 = 1;
    let mut proximo_numero : i32 ;


    while contador < quantidade_numeros{

        proximo_numero = numero_anterior + numero_atual;
        numero_anterior = numero_atual;
        numero_atual = proximo_numero;

        println!("{}", proximo_numero);
        contador = contador + 1;

    }*/

    println!("Sequência de Fibonacci:");

    Fibonacci(0);

    

}

fn Fibonacci(contador : i32) -> i32 {
  
    let mut numero_anterior : i32 = -1;
    let mut numero_atual : i32 = 1;
    let mut proximo_numero : i32 ;

    if contador > 13 {
       return contador;
  } else {
    proximo_numero = numero_anterior + numero_atual;
    numero_anterior = numero_atual;
    numero_atual = proximo_numero;

    return println!("{}", proximo_numero);
    Fibonacci(contador + 1);

  }

}
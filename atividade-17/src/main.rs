/*Elaborar um programa que apresente os resultados das potências do valor de base 3, elevando a um expoente que varie do valor 0 até o valor 15.

Restrição: Não utilize a função de exponenciação da biblioteca i32.  */

fn main() {
    println!("Resultados das potências do valor de base 3, elevando a um expoente que varie do valor 0 até o valor 15");

    let base: i32 = 3;
    let mut contador: i32 = 0;
    let mut potencia: i32;

    while contador <= 15 {
        potencia = base * contador;

        println!(
            "O resultado da potência do valor de base {}, elevado ao expoente {}: {}",
            base, contador, potencia
        );

        contador = contador + 1;
    }
}

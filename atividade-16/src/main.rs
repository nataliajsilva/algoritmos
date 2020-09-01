/*Construir um programa que apresente todos os valores numéricos divisíveis por 4 e menores que 200.

Restrição: Utilize o laço condicional pós-teste. */

fn main() {
    println!("Números divisivéis por 4 e menores que 200");

    for numero in 0..200 {
        if numero % 4 == 0 {
            println!("{}", numero);
        }
    }
}

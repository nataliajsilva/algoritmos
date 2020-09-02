/*Construir um programa que apresente todos os valores numéricos divisíveis por 4 e menores que 200.

Restrição: Utilize o laço condicional pós-teste. */

fn main() {
    println!("Números divisivéis por 4 e menores que 200");

    let mut count = 0;

    loop {
        count += 1;

        if count % 4 == 0 {
            println!("{}", count);
        }

        if count >= 200 {
            break;
        }
    }
}

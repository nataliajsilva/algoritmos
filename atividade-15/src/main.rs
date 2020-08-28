/*Elaborar um programa que apresente todos os valores numéricos inteiros ímpares situados na faixa de 0 a 20.

Restrição: Utilize o laço incondicional.*/

//Pseudocódigo
//Construir um laço onde verifique quais são os número ímpares entra 0 e 20
//Exibir na tela os números ímpares encontrados


fn main() {
    
    let numeros = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];

    println!("Os números ímpares que existem entre 0 e 20 são:");

    for elemento in numeros.iter() {

        if elemento % 2 == 1 {

            println!("{}", elemento);

        } 
    }
}

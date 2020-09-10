
/*Elaborar um programa que leia 10 elementos (valores reais) para temperatura em graus Celsius e armazene esses valores em um vetor A. O programa ao final deve apresentar a menor, a maior e a média das temperaturas lidas.*/

use std::io;

fn main() {

    println!("Exibindo a menor, a maior e a média das temperaturas");

    let mut a: [f32; 10] = [0.0; 10];
    let mut temperaturas_indice_a: usize = 0;

    while temperaturas_indice_a <= 9 {
        println!("Digite uma temperatura em graus Celsius para inserir no vetor A");
        let mut temperaturas_a: String = String::new();
        io::stdin()
            .read_line(&mut temperaturas_a)
            .expect("Falha ao ler o valor");
        let temperaturas_a: f32 = converter_string_para_f32(temperaturas_a);
    
        a[temperaturas_indice_a] = temperaturas_a;
        temperaturas_indice_a = temperaturas_indice_a + 1;
    }

    let mut menor_temperatura :f32 = a[0];
    let mut maior_temperatura :f32 = a[0];
    
    //calcular menor
    for i in a.iter(){
         
        if i < &menor_temperatura {
            menor_temperatura = *i;
        } 
    }
    
    //calcular maior
    for i in a.iter(){
         
        if i > &maior_temperatura {
            maior_temperatura = *i;
        } 
    }

    //calcular média
    let mut total :f32 = 0.0;

    for i in a.iter(){
         
        total = total + i;
        } 

    let media_temperaturas :f32 = total / 10.0 ;

    //exibir a menor, a maior e a média
    println!("A menor temperatura lida foi {}° , a maior temperatura lida foi {}° e a media das temperaturas é {}°", 
    menor_temperatura, maior_temperatura, media_temperaturas);

    fn converter_string_para_f32(uma_string: String) -> f32 {
        uma_string
           .trim()
           .parse()
           .expect("Falha ao converter o valor")
     }

}

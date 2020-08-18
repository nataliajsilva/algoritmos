/*Escrever um programa que leia o nome de um aluno e as notas de três provas, P1, P2 e P3. O aluno é aprovado se possuir média aritmética (MD) acima de 7, estará em recuperação entre 5 e 7, e será reprovado abaixo de 5. Imprimir todos os dados capturados, e a situação do aluno. */

//Pseudocódigo
//Pedir ao usuário o nome dele
//Pedir ao usuário a nota da prova 1
//Pedir ao usuário a nota da prova 2
//Pedir ao usuário a nota da prova 3
//Calcular a média aritmética utilizando as notas das 3 provas
//Verificar a situação do aluno (aprovado, em recuperação ou reprovado)
//Exibir as notas e informar ao usuário a situação do aluno

use std::io;

fn main() {
    
    println!("Calculando média aritmética");

    println!("Digite o seu nome");
    let mut nome : String = String::new();
    io::stdin()
       .read_line(&mut nome)
       .expect("Falha ao ler o valor");


    println!("Digite sua nota da P1");
    let mut p1 : String = String::new();
    io::stdin()
       .read_line(&mut p1)
       .expect("Falha ao ler o valor");

    let p1 : f32 = converter_string_para_f32(p1);


    println!("Digite sua nota da P2");
    let mut p2 : String = String::new();
    io::stdin()
        .read_line(&mut p2)
        .expect("Falha ao ler o valor");
    
    let p2 : f32 = converter_string_para_f32(p2);


    println!("Digite sua nota da P3");
    let mut p3 : String = String::new();
    io::stdin()
       .read_line(&mut p3)
       .expect("Falha ao ler o valor");
    
    let p3 : f32 = converter_string_para_f32(p3);

    
    let media : f32 = (p1 + p2 + p3)/3f32;

    
    if media > 7.0 {
        print!("{}, com base nas notas das provas P1: {}, P2: {} e P3: {}, a média é {}. Sendo assim, a situação do aluno é: APROVADO ",nome, p1, p2, p3, media);

    } else if media >= 5.0 && media <=7.0 {
        print!("{}, com base nas notas das provas P1: {}, P2: {} e P3: {}, a média é {}. Sendo assim, a situação do aluno é: EM RECUPERAÇÃO ",nome, p1, p2, p3, media);

    } else {
        print!("{}, com base nas notas das provas P1: {}, P2: {} e P3: {}, a média é {}. Sendo assim, a situação do aluno é: REPROVADO ",nome, p1, p2, p3, media);
    }


    fn converter_string_para_f32(uma_string: String) -> f32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }

}

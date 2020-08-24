// Contar de 0 até 10 utilizando função (reutilização)

fn main() {
    
    conte_ate(1,20);
}

fn conte_ate(inicio : i32 ,fim : i32) -> i32 {
    if inicio > fim {
        return inicio;
    } else {
        println!("{}", inicio);
        conte_ate(inicio + 1, fim )

    }
}

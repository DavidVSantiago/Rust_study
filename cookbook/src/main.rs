use unilib::{aleatorios::*};

fn main() {

    let mut rand = GeradorAleatorio::new();
    let var = rand.gera_rand_primo(Intervalo::Entre(1, 1000));
    println!("{}", var);

}

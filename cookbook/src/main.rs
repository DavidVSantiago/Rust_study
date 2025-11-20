use cookbook::aleatorios::*;
use std::time::Instant;

fn main() {

    let mut gerador = GeradorAleatorio::new();

    let start_time = Instant::now();
    let senha = gerador.rand_senha(100,TipoSenha::Total);
    let elapsed = start_time.elapsed();

    println!("Senha gerada: {}", senha);
    println!("Tempo total gasto: {} ns", elapsed.as_nanos());

}

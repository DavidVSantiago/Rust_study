
use cookbook::aleatorios::*;
use std::time::Instant;

fn main() {

    let tam =500_000;
    let mut vetor:Vec<String> = Vec::with_capacity(tam);
    
    let mut gerador = GeradorAleatorio::new();

    let start_time = Instant::now();
    for _ in 0..tam{
        let senha = gerador.rand_senha(100,TipoSenha::Total);
        vetor.push(senha);
    }

    let elapsed = start_time.elapsed();

    println!("Tempo total gasto: {} ns", elapsed.as_millis());

}

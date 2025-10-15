use rand::Rng;
use rand_distr::{Distribution, Uniform};

fn main(){
    let mut rng = rand::rng(); // obtem o gerador de aleatório
    
    // retorna um psudo aleaótio sobre todo o alcance do tipo
    let rn_01: u32 = rng.random();
    println!("Número aleatório: {}",rn_01);

    // retorna um psudo aleaótio dentro de um intervalo
    let rn_02 = rng.random_range(0..10); // intervalo inteiro entre 0 e 10
    println!("Inteiro: {rn_02}");
    let rn_03 = rng.random_range(0.0..10.0); // intervalo float entre 0.0 e 10.0
    println!("Float: {rn_03}");

    // distribuição uniforme
    let dado = Uniform::new_inclusive(1,6).expect("Falha em criar distribuição uniforme!");

    loop{
        let throw = dado.sample(&mut rng);
        println!("Rolando o dado: {throw}");
        if throw == 6 { break; }
    }


}

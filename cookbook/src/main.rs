use cookbook::{aleatorios::*, ordenacao::*};
use std::{ptr::eq, time::Instant};

#[derive(Debug)] // para poder imprimir
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age
        }
    }
}
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
       self.age==other.age
    }
}
impl Eq for Person {} // marker trait

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.age.partial_cmp(&other.age)
    }
}

// Passo 4: Definir a Ordem Total (necessário para o método .sort())
impl Ord for Person {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Delega a comparação para o u32 (age)
        self.age.cmp(&other.age)
    }
}

fn main() {

    let mut array = vec![
        Person::new("Zoe", 25),
        Person::new("Al", 60),
        Person::new("Jucelia", 24),
        Person::new("Ana", 32),
        Person::new("Ziraldo", 89),
        Person::new("Amarildo", 54),
        Person::new("Pedro", 21),
        Person::new("Manoela", 75),
        Person::new("David", 45),
        Person::new("Daniela", 12),
    ];
    
    println!("Array antes: {:?}",array);

    let inicio = Instant::now();
    ordena_vetor(&mut array);
    let duracao = inicio.elapsed().as_nanos();
    println!("Array depois: {:?}",array);
    println!("Tempo total para ordenar: {} nanosegundos", duracao);
}

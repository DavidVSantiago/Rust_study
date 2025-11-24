use std::{env, fs, process,error::Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = match Config::build(&args) {
        Ok(config) => config,
        Err(err) => {
            println!("Erro: {}", err);
            process::exit(1);
        }
    };
    
    println!("Procurando pela palavra {} ", config.palavra_chave);
    println!("No arquivo {} ", config.nome_arquivo);

    if let Err(e) = run(config) {
        println!("Erro ao executar a função run: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let conteudo = fs::read_to_string(config.nome_arquivo)?;
    println!("Conteúdo do arquivo:\n{}", conteudo);

    Ok(())
}

struct Config{
    palavra_chave: String,
    nome_arquivo: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("O programa deve ser executado com, no mínimo, 2 parâmetros!");
        }
        let palavra_chave = args[1].clone();
        let nome_arquivo = args[2].clone();
        Ok(Self{palavra_chave,nome_arquivo})
    }
    
}
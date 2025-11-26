use std::{env, fs, process,error::Error};
use minigrep::{pesquisar, pesquisar_sem_caso}; // importa a função da binário de bibliotecas

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = match Config::build(&args) { // testa erro de carregamento de arquivo
        Ok(config) => config,
        Err(err) => {
            eprintln!("Erro: {}", err);
            process::exit(1);
        }
    };
    
    println!("Procurando pela palavra '{}' ", config.palavra_chave);
    println!("No arquivo '{}' ", config.nome_arquivo);

    if let Err(e) = run(config) {
        println!("Erro ao executar a função run: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let conteudo = fs::read_to_string(config.nome_arquivo)?; // se houver erro, lança para cima
    
    let resultados = if config.ignore_case{
        pesquisar_sem_caso(&config.palavra_chave, &conteudo)
    }else {
        pesquisar(&config.palavra_chave, &conteudo)
    };

    for line in resultados{
        println!("{line}");
    }

    Ok(())
}



// -----------------------------------------------------------------------------------------
struct Config{
    pub palavra_chave: String,
    pub nome_arquivo: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("O programa deve ser executado com, no mínimo, 2 parâmetros!");
        }
        let palavra_chave = args[1].clone();
        let nome_arquivo = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Self{palavra_chave,nome_arquivo,ignore_case})
    }
    
}
use std::process;

use cookbook::{arquivos::*, tempo::*, textos::*};

fn main() {

    let mut crono = Cronometro::new();

    let nome_arquivo = String::from("livro.txt");
    let result = match ler_arquivo_utf8(&nome_arquivo) {
        Ok(s) => s,
        Err(e) => {
            print!("{}", e);
            process::exit(1)
        }
    };
    let ocorrencia = "Et";
    crono.marcar_inicio();
    let qtd = conta_ocorrencias_texto(&result, ocorrencia, true);
    crono.marcar_fim();

    println!("{} palavras", qtd);
    println!(
        "O programa demorou {} segundos!",
        crono.obter_tempo_segundos()
    );
    println!(
        "O programa demorou {} milisegundos!",
        crono.obter_tempo_mili()
    );
    println!(
        "O programa demorou {} nanosegundos!",
        crono.obter_tempo_nano()
    );
}

use std::process;

use cookbook::{ arquivos::*, textos::*, tempo::*};


fn main() {
    let mut crono = Cronometro::new();

    crono.marcar_inicio();
    let nome_arquivo = String::from("proibida_para_mim.txt");
    let result= match ler_arquivo_utf8(&nome_arquivo) {
        Ok(s) => s,
        Err(e) => { print!("{}",e); process::exit(1)},
    };
    std::thread::sleep(std::time::Duration::from_millis(150));
    let ocorrencia = "e";
    let qtd = conta_ocorrencias_texto(&result, ocorrencia, true);
    crono.marcar_fim();
    println!("O texto '{}' aparece '{}' vezes",ocorrencia,qtd);
    println!("O programa demorou {} segundos!",crono.obter_tempo_segundos());
    println!("O programa demorou {} milisegundos!",crono.obter_tempo_mili());
    println!("O programa demorou {} nanosegundos!",crono.obter_tempo_nano());
}


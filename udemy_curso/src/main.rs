use std::thread::sleep;
use std::time::Duration;
use unilib::{arquivos::*, tempo::*, textos::*};

// TRECHO DAS VIAS APÓS O CRUZAMENTO (FORA DE CONTROLE)
const _VIAH_MARGEM: f64 = 15.0;
const _VIAV_MARGEM: f64 = 15.0;

// LARGURA DAS VIAS
const _VIAH_LARGURA: f64 = 4.0;
const _VIAV_LARGURA: f64 = 4.0;

// TRECHO DAS VIAS ANTES DO CRUZAMENTO (SOB CONTROLE)
const _VIAH_PERIMETRO: f64 = 150.0;
const _VIAV_PERIMETRO: f64 = 150.0;

// DIMENSÕES DOS CARROS
const _CARRO_LARGURA: f64 = 2.0;
const _CARRO_COMPRIMENTO: f64 = 4.0;

// VELOCIDADE DOS CARROS
const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0); // 200 km/h em m/s

// ACELERAÇÃO MÁXIMA DOS CARROS
const ACELERACAO_MAXIMA: f64 = 3.0; // 3 m/s²

// ACELERAÇÃO MÍNIMA DOS CARROS (frenagem)
const ACELERACAO_MINIMA: f64 = -10.0; // -10 m/s²


/// simula 2 carros até sairem do perímetro controlado (ou colidirem).
fn simula_carros(via_carro1:char, acel_carro1: f64, via_carro2:char, acel_carro2: f64) -> bool{
    
    // estrutura carro 1
    let chassi1 = 1111; // id do carro
    let via1 = via_carro1; // a via deste caro 
    let _acel_max1 = ACELERACAO_MAXIMA;
    let _acel_min1 = ACELERACAO_MINIMA;
    let vel_max1 = VELOCIDADE_MAXIMA;
    let comprimento1 = _CARRO_COMPRIMENTO;
    let mut pos_atual1 = -80.0; // vai mudando a cada qudro (zero = o cruzamento)
    let mut vel_atual1 = 0.0;
    let acel_atual1: f64;

    // estrutura carro 2
    let chassi2 = 2222; // id do carro
    let via2 = via_carro2; // a via deste caro 
    let _acel_max2 = ACELERACAO_MAXIMA;
    let _acel_min2 = ACELERACAO_MINIMA;
    let vel_max2 = VELOCIDADE_MAXIMA;
    let comprimento2 = _CARRO_COMPRIMENTO;
    let mut pos_atual2 = -100.0; // vai mudando a cada qudro (zero = o cruzamento)
    let mut vel_atual2 = 0.0;
    let acel_atual2: f64;

    acel_atual1 = acel_carro1;
    acel_atual2 = acel_carro2;

    print!("Inicio da simulação");
    let mut tickms = 100; // tempo passado
    let tick = tickms as f64 /1000.0;
    loop { // o gameloop
        sleep(Duration::from_millis(tickms)); // pausa da tread

        // update do carro 1
        let old_position = pos_atual1;
        
        pos_atual1 = pos_atual1 + (vel_atual1 * tick) + (acel_atual1 * tick * tick / 2.0);
        vel_atual1 = vel_atual1 + acel_atual1 * (tick);


        // restrições --------------------------
        if pos_atual1 < old_position {pos_atual1=old_position;} // evita ré
        if vel_atual1 < 0.0 {vel_atual1=0.0;} // limiar inferior de velocidade
        if vel_atual1 > vel_max1 {vel_atual1=vel_max1;} // limiar superior de velocidade

        println!("Carro {} na posição {}:{}, velocidade {}, aceleração {}", chassi1, via1, pos_atual1, vel_atual1, acel_atual1);

        // update do carro 2
        let old_position = pos_atual2;
        
        pos_atual2 = pos_atual2 + (vel_atual2 * tick) + (acel_atual2 * tick * tick / 2.0);
        vel_atual2 = vel_atual2 + acel_atual2 * (tick);


        // restrições --------------------------
        if pos_atual2 < old_position {pos_atual2=old_position;} // evita ré
        if vel_atual2 < 0.0 {vel_atual2=0.0;} // limiar inferior de velocidade
        if vel_atual2 > vel_max2 {vel_atual2=vel_max2;} // limiar superior de velocidade

        println!("Carro {} na posição {}:{}, velocidade {}, aceleração {}", chassi2, via2, pos_atual2, vel_atual2, acel_atual2);

        // detecta colisão na via H
        if via1 == 'H' && via2 == 'H'{ // ambos carros devem estar na mesma via
            if colisaoLongitudinal(pos_atual1,comprimento1,pos_atual2){
                println!("Colisão detectada na via H");
                return true; // qual o significado do retorno nessa função??
            }
        }

        // detecta colisão na via V
        if via1 == 'V' && via2 == 'V'{ // ambos carros devem estar na mesma via
            if colisaoLongitudinal(pos_atual1,comprimento1,pos_atual2){
                println!("Colisão detectada na via V");
                return true; // qual o significado do retorno nessa função??
            }
        }

        // detecta colisão no cruzamento
        if via1 != via2 { // ambos os carros têm que ser de origens (vias) distintas 
            if dentroCruzamento(pos_atual1,comprimento1,via1) &&
                dentroCruzamento(pos_atual2,comprimento2,via2){
                println!("Colisão detectada no cruzamento");
                return true;
            }
        }
        // verifica se o carro 1 saiu do sistema
        if pos_atual1 > comprimento1 + if via1 == 'H' { _VIAV_LARGURA } else { _VIAH_LARGURA } {
            break;
        }
        // verifica se o carro 2 saiu do sistema
        if pos_atual2 > comprimento2 + if via2=='H' { _VIAV_LARGURA} else { _VIAH_LARGURA} {
            break;
        }

    }
    return false;

}

fn colisaoLongitudinal(pos_frente: f64,comprimento: f64,pos_atras: f64) -> bool{
    return pos_frente - comprimento <= pos_atras;
}

fn dentroCruzamento(pos: f64,comprimento: f64,via: char) -> bool{
    return pos > 0.0 &&
            pos <= comprimento + if via == 'H' { _VIAV_LARGURA} else { _VIAH_LARGURA};
}

fn main() {
    let mut cron = Cronometro::new();
    cron.marcar_inicio();
    println!("Inicio da simulação!");
    simula_carros('H', ACELERACAO_MAXIMA/10.0, 'V', ACELERACAO_MAXIMA);
    println!("Fim da simulação!");
    cron.marcar_fim();
    println!("Tempo de simulação: {} segundos", cron.obter_tempo_segundos());
}

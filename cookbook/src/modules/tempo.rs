use std::time::Instant;

// ************************************************************************************************
// --- API PÚBLICA ---
// ************************************************************************************************

pub struct Cronometro{
    inicio: Instant,
    fim: Instant,
}

impl Cronometro{
    pub fn new() -> Self{
        Self{
            inicio: Instant::now(),
            fim: Instant::now(),
        }
    }

    pub fn marcar_inicio(&mut self){
        self.inicio = Instant::now();
    }

    pub fn marcar_fim(&mut self){
        self.fim = Instant::now();
    }

    pub fn obter_tempo_segundos(&self) -> f32{
        self.fim.duration_since(self.inicio).as_secs_f32()
    }

    pub fn obter_tempo_mili(&self) -> u128{
        self.fim.duration_since(self.inicio).as_millis()
    }

    pub fn obter_tempo_nano(&self) -> u128{
        self.fim.duration_since(self.inicio).as_nanos()
    }
}

// ************************************************************************************************
// --- MÉTODOS PRIVADOS ---
// ************************************************************************************************
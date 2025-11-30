use std::time::Instant;

// ************************************************************************************************
// --- API PÚBLICA ---
// ************************************************************************************************

/// Estrutura responsável pela **medição precisa de tempo de execução**.
///
/// A `Cronometro` fornece uma abstração de alto nível para capturar intervalos de
/// tempo com alta resolução, permitindo a medição de duração em segundos,
/// milissegundos e nanossegundos.
///
/// A estrutura encapsula dois instantes de tempo:
///
/// * `inicio` — marca o momento inicial da medição.
/// * `fim` — marca o momento final da medição.
///
/// # Finalidade
///
/// Este tipo é ideal para medições de desempenho, instrumentação de código,
/// análise de tempo de execução e benchmarks simples.
///
/// Ele utiliza o tipo `Instant` da biblioteca padrão, que é monotônico e imune
/// a alterações no relógio do sistema.
///
/// # Observações
///
/// * Todas as medições são realizadas utilizando tempo monotônico.
/// * Chamadas sucessivas a `marcar_inicio` reiniciam a contagem.
/// * Não há garantia de que `marcar_fim` seja chamado após `marcar_inicio`,
///   ficando sob responsabilidade do usuário manter a ordem lógica das chamadas.
///
/// # Exemplo
///
/// ```
/// let mut c = Cronometro::new();
/// c.marcar_inicio();
///
/// // código a ser medido
///
/// c.marcar_fim();
/// println!("Tempo: {} ms", c.obter_tempo_mili());
/// ```
pub struct Cronometro {
    inicio: Instant,
    fim: Instant,
}

impl Cronometro {
    /// **Cria** uma nova instância do cronômetro.
    ///
    /// Inicializa os campos `inicio` e `fim` com o instante atual, garantindo
    /// que a estrutura esteja em estado válido imediatamente após a construção.
    ///
    /// # Retorno
    ///
    /// Retorna uma nova instância de `Cronometro`
    pub fn new() -> Self {
        Self {
            inicio: Instant::now(),
            fim: Instant::now(),
        }
    }

    /// **Marca** o instante inicial da medição.
    ///
    /// Atualiza o campo `inicio` com o tempo atual, reiniciando a contagem
    /// do intervalo.
    ///
    /// # Observações
    ///
    /// Caso este método seja chamado múltiplas vezes, apenas o último valor
    /// será considerado como início válido.
    pub fn marcar_inicio(&mut self) {
        self.inicio = Instant::now();
    }

    /// **Marca** o instante final da medição.
    ///
    /// Atualiza o campo `fim` com o tempo atual, encerrando a contagem
    /// do intervalo.
    ///
    /// # Observações
    ///
    /// A medição somente será válida se este método for chamado após
    /// `marcar_inicio`.
    pub fn marcar_fim(&mut self) {
        self.fim = Instant::now();
    }

    /// Retorna a duração do intervalo medido em **segundos**.
    ///
    /// # Retorno
    ///
    /// Um valor `f32` representando a duração em segundos.
    pub fn obter_tempo_segundos(&self) -> f32 {
        self.fim.duration_since(self.inicio).as_secs_f32()
    }

    /// Retorna a duração do intervalo medido em **milissegundos**.
    ///
    /// # Retorno
    ///
    /// Um valor inteiro (`u128`) representando a duração em milissegundos.
    pub fn obter_tempo_mili(&self) -> u128 {
        self.fim.duration_since(self.inicio).as_millis()
    }

    /// Retorna a duração do intervalo medido em **nanossegundos**.
    ///
    /// # Retorno
    ///
    /// Um valor inteiro (`u128`) representando a duração em nanossegundos.
    pub fn obter_tempo_nano(&self) -> u128 {
        self.fim.duration_since(self.inicio).as_nanos()
    }
}

use std::time::{SystemTime, UNIX_EPOCH}; // para obter o relógio para o seed
use tinyrand::{Rand, RandRange, Seeded, StdRand};
use crate::mat::calc_primo;

/// Estrutura responsável pela **geração de números pseudoaleatórios**.
///
/// A `GeradorAleatorio` encapsula um gerador pseudoaleatório (`StdRand`) que é utilizado
/// como núcleo para todas as operações de geração de valores aleatórios no sistema.
///
/// Esta estrutura funciona como uma camada de abstração sobre o gerador interno,
/// centralizando o controle da semente (*seed*) e a lógica de geração, de forma que
/// toda a aplicação utilize um único mecanismo consistente para produção de dados
/// pseudoaleatórios.
///
/// # Exemplos
///
/// ```
/// let mut g = GeradorAleatorio::new();
/// let n = g.gera_rand(Intervalo::Total);
/// let senha = g.gera_rand_senha(12, TipoSenha::NumerosLetras);
/// ```
pub struct GeradorAleatorio {
    rng: StdRand, // O gerador de números pseudoaleatórios é armazenado dentro da struct.
}

impl GeradorAleatorio {
    /// **Cria** uma nova instância do gerador de números aleatórios.
    ///
    /// Esta função inicializa um gerador pseudoaleatório utilizando uma semente
    /// baseada no tempo atual do sistema (tempo decorrido desde a época UNIX).
    /// Isso garante que cada instância criada tenha uma semente distinta, evitando
    /// a repetição de sequências pseudoaleatórias entre execuções distintas do programa.
    ///
    /// A semente é obtida através da quantidade de nanossegundos decorridos desde
    /// `UNIX_EPOCH`, sendo convertida para `u64` e utilizada na inicialização interna
    /// do gerador (`StdRand`).
    ///
    /// # Detalhes de Implementação
    ///
    /// * O relógio do sistema é consultado via `SystemTime::now()`.
    /// * A diferença em relação à época UNIX é calculada com `duration_since(UNIX_EPOCH)`.
    /// * O valor em nanossegundos é usado como semente pseudoaleatória.
    /// * Um erro é acionado se o relógio do sistema estiver inconsistente (ex: tempo retrocedido).
    ///
    /// # Retorno
    ///
    /// Retorna uma nova instância de `GeradorAleatorio` devidamente inicializada e
    /// pronta para geração de números pseudoaleatórios.
    pub fn new() -> Self {
        let start = SystemTime::now(); // Captura o tempo atual do sistema
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("O tempo andou para trás"); // Tratamento de erro básico
        let seed = since_the_epoch.as_nanos() as u64; // gera um seed único baseado no relógio
        GeradorAleatorio {
            rng: StdRand::seed(seed),
        } // Usa `StdRand::default()` para inicialização, que é tipicamente a semente baseada no tempo ou hardware.
    }

    // ************************************************************************************************
    // --- API PÚBLICA ---
    // ************************************************************************************************

    /// **Gera** um número inteiro pseudoaleatório dentro de um intervalo especificado.
    ///
    /// Esta função atua como uma interface unificada para geração de números aleatórios,
    /// delegando a lógica de geração conforme o tipo de intervalo informado.
    ///
    /// O intervalo é definido através do enum `Intervalo`, podendo assumir uma das
    /// seguintes variações:
    ///
    /// * `Intervalo::Total` — gera um valor dentro de todo o intervalo válido do tipo `u32`.
    /// * `Intervalo::Entre(a, b)` — gera um valor inteiro pseudoaleatório no intervalo
    ///   inclusivo delimitado por `a` e `b`.
    ///
    /// # Argumentos
    ///
    /// * `i` - O intervalo no qual o número aleatório será gerado.
    ///
    /// # Retorno
    ///
    /// Retorna um valor do tipo `u32` gerado pseudoaleatoriamente dentro do intervalo
    /// especificado.
    ///
    pub fn gera_rand(&mut self, i: Intervalo) -> u32 {
        match i {
            Intervalo::Total => self.gera_rand_total(), // retorna o intervalo total
            Intervalo::Entre(a, b) => self.gera_rand_entre(a, b),
        }
    }

    /// **Gera** um número primo pseudoaleatório dentro de um intervalo especificado.
    ///
    /// Esta função utiliza a lógica de geração aleatória e a combina com uma rotina de
    /// busca para garantir que o valor retornado seja um número primo. O processo de
    /// busca é realizado incrementando o número aleatório inicial até que a função
    /// `calc_primo` retorne verdadeiro.
    ///
    /// O intervalo é definido através do enum `Intervalo`, podendo assumir uma das
    /// seguintes variações:
    ///
    /// * `Intervalo::Total` — gera um valor aleatório dentro de todo o intervalo válido do tipo `u32`
    ///   e incrementa a partir desse ponto até encontrar o primeiro primo.
    /// * `Intervalo::Entre(a, b)` — gera um valor aleatório dentro do intervalo inclusivo
    ///   delimitado por `a` e `b`. A busca por um primo é feita por incremento, e se o
    ///   limite superior (`b`) for atingido, a busca recomeça a partir de `a` (comportamento de 'wrap-around').
    ///
    /// **Nota sobre a Eficiência:** O número de iterações (`valor+=1`) é regido pela
    /// densidade dos números primos, que é relativamente alta.
    ///
    /// # Argumentos
    ///
    /// * `i` - O intervalo no qual o número primo aleatório será gerado e a busca iniciada.
    ///
    /// # Retorno
    ///
    /// Retorna um valor primo do tipo `u32` encontrado após a busca iniciada a partir de
    /// um ponto pseudoaleatório dentro do intervalo especificado.
    ///
    pub fn gera_rand_primo(&mut self, i: Intervalo) -> u32 {
         match i {
            Intervalo::Total => {
                let mut valor = self.gera_rand_total();
                while !calc_primo(valor) {valor+=1;}
                valor
            },
            Intervalo::Entre(mut a, mut b) => {
                if a>b {let aux=b; b=a; a=aux;} // troca a e b
                let mut valor = self.gera_rand_entre(a, b+1);
                while !calc_primo(valor) {
                    valor+=1;
                    if valor>=b {valor =a;}
                }
                valor
            },
        }
    }

    /// **Gera** uma senha pseudoaleatória com base em um conjunto de caracteres permitido.
    ///
    /// Esta função constrói uma string de tamanho fixo contendo caracteres escolhidos
    /// pseudoaleatoriamente a partir de um conjunto definido pelo tipo `TipoSenha`.
    ///
    /// O tipo da senha determina quais categorias de caracteres poderão ser utilizadas:
    ///
    /// * `TipoSenha::Numeros` — apenas dígitos numéricos.
    /// * `TipoSenha::LetrasMaiusculas` — apenas letras maiúsculas.
    /// * `TipoSenha::LetrasMinusculas` — apenas letras minúsculas.
    /// * `TipoSenha::Letras` — letras maiúsculas e minúsculas.
    /// * `TipoSenha::NumerosLetras` — mistura de números e letras.
    /// * `TipoSenha::Especiais` — apenas caracteres especiais.
    /// * `TipoSenha::Total` — inclui letras, números e caracteres especiais.
    ///
    /// # Argumentos
    ///
    /// * `tam` — tamanho da senha a ser gerada (quantidade de caracteres).
    /// * `t` — define o tipo de caracteres que serão utilizados na composição da senha.
    ///
    /// # Retorno
    ///
    /// Retorna uma `String` contendo a senha gerada, com comprimento exatamente igual a `tam`.
    pub fn gera_rand_senha(&mut self, tam: usize, t: TipoSenha) -> String {
        let caracteres_permitidos: &[u8] = match t {
            TipoSenha::Numeros => &N_ARR,
            TipoSenha::LetrasMaiusculas => &LMAI_ARR,
            TipoSenha::LetrasMinusculas => &LMIN_ARR,
            TipoSenha::Letras => &L_ARR,
            TipoSenha::NumerosLetras => &NL_ARR,
            TipoSenha::Especiais => &E_ARR,
            TipoSenha::Total => &T_ARR,
        };
        let c_tam: u32 = caracteres_permitidos.len() as u32; // a quantidade de caracteres no array de caracteres permitidos
        let mut senha = vec!['#'; tam]; // um array de tamanho t
        // percorre cada caracter para colocar os aleatórios
        for item in senha.iter_mut() {
            // obtem um indice dos caractere aleatórios permitidos
            let indice: usize = self.gera_rand(Intervalo::Entre(0, c_tam - 1)) as usize;
            *item = caracteres_permitidos[indice] as char;
        }
        // converte para string e retorna
        senha.into_iter().collect()
    }

    // ************************************************************************************************
    // --- MÉTODOS PRIVADOS ---
    // ************************************************************************************************

    /// Retorna um número aleatório de 32 bits no intervalo total [0, u32::MAX].
    fn gera_rand_total(&mut self) -> u32 {
        self.rng.next_u32()
    }

    /// Retorna um número aleatório no intervalo [a, b] inclusivo.
    fn gera_rand_entre(&mut self, a: u32, b: u32) -> u32 {
        self.rng.next_range(a..b)
    }
}

/* Os arrays estáticos abaixo armazenam os intervalos de código ASCII necessários para a geração de senhas */
const N_ARR: [u8; 10] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57];
const LMAI_ARR: [u8; 26] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
    89, 90,
];
const LMIN_ARR: [u8; 26] = [
    97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115,
    116, 117, 118, 119, 120, 121, 122,
];
const L_ARR: [u8; 52] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114,
    115, 116, 117, 118, 119, 120, 121, 122,
];
const NL_ARR: [u8; 62] = [
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78,
    79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106,
    107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
];
const E_ARR: [u8; 32] = [
    33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 58, 59, 60, 61, 62, 63, 64, 91, 92,
    93, 94, 95, 96, 123, 124, 125, 126,
];
const T_ARR: [u8; 94] = [
    33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    123, 124, 125, 126,
];

/// Representa os **intervalos possíveis** para geração de números pseudoaleatórios.
///
/// O enum `Intervalo` define as políticas de distribuição dos valores gerados,
/// permitindo solicitar números.
///
/// # Variantes
///
/// * `Total` — gera valores em todo o intervalo possível do tipo `u32`.
/// * `Entre(a, b)` — gera valores inteiros pseudoaleatórios no intervalo fechado
///   definido pelos limites `a` e `b`, inclusive.
///
/// # Exemplos
///
/// ```
/// let mut g = GeradorAleatorio::new();
/// let x = g.gera_rand(Intervalo::Entre(10, 20));
/// let y = g.gera_rand(Intervalo::Total);
pub enum Intervalo {
    Total,
    Entre(u32, u32),
}

/// Define as **políticas de composição de senhas pseudoaleatórias**.
///
/// O enum `TipoSenha` especifica quais categorias de caracteres poderão ser
/// utilizadas na geração de senhas, possibilitando diferentes níveis de
/// complexidade e restrição.
///
/// # Variantes
///
/// * `Numeros` — apenas dígitos numéricos.
/// * `LetrasMaiusculas` — apenas letras maiúsculas (A–Z).
/// * `LetrasMinusculas` — apenas letras minúsculas (a–z).
/// * `Letras` — letras maiúsculas e minúsculas.
/// * `NumerosLetras` — combina números e letras.
/// * `Especiais` — apenas caracteres especiais (ex: `!@#$%`).
/// * `Total` — todos os caracteres disponíveis.
///
/// # Uso
///
/// O tipo da senha é especificado como argumento do método `gera_rand_senha`.
///
/// # Exemplos
///
/// ```
/// let mut g = GeradorAleatorio::new();
/// let senha = g.gera_rand_senha(16, TipoSenha::Total);
/// let simples = g.gera_rand_senha(6, TipoSenha::Numeros);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum TipoSenha {
    Numeros,
    LetrasMaiusculas,
    LetrasMinusculas,
    Letras,
    NumerosLetras,
    Especiais,
    Total,
}

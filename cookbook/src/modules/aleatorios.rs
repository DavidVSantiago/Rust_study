use std::time::{SystemTime, UNIX_EPOCH}; // para obter o relógio para o seed
use tinyrand::{Rand, RandRange, Seeded, StdRand};

// --- TIPOS PÚBLICOS ---

/// Representa o tipo de caracteres permitidos para a geração de senhas.
#[derive(Debug, Clone, Copy)]
pub enum TipoSenha {
    Numeros, LetrasMaiusculas, LetrasMinusculas, Letras, 
    NumerosLetras, Especiais, Total,
}

pub enum Intervalo {
    Total,
    Entre(u32, u32),
}

// --- ESTRUTURA PRINCIPAL ---

/// Encapsula a lógica de geração de números aleatórios e senhas.
pub struct GeradorAleatorio {
    rng: StdRand, // O gerador de números pseudoaleatórios é armazenado dentro da struct.
}

impl GeradorAleatorio {
    pub fn new() -> Self { // --- CONSTRUTOR ---
        
        let start = SystemTime::now(); // Captura o tempo atual do sistema
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("O tempo andou para trás"); // Tratamento de erro básico
        let seed = since_the_epoch.as_nanos() as u64; // gera um seed único baseado no relógio

        GeradorAleatorio { rng: StdRand::seed(seed), } // Usa `StdRand::default()` para inicialização, que é tipicamente a semente baseada no tempo ou hardware.
    }
    
    // ************************************************************************************************
    // --- API PÚBLICA ---
    // ************************************************************************************************

    /// gera um numero aleatório em um intervalo informado
    pub fn rand(&mut self, i:Intervalo) -> u32 {
        match i {
            Intervalo::Total => self.rand_total(), // retorna o intervalo total
            Intervalo::Entre(a, b) => self.rand_entre(a, b),
        }
    }

    /// gera uma senha com 'tam' caracteres e de tipos especificados por 'TipoSenha'
    pub fn rand_senha(&mut self, tam: usize, t: TipoSenha) -> String{
        let caracteres_permitidos: &[u8] = match t {
            TipoSenha::Numeros => &N_ARR,
            TipoSenha::LetrasMaiusculas => &LMAI_ARR,
            TipoSenha::LetrasMinusculas => &LMIN_ARR,
            TipoSenha::Letras => &L_ARR,
            TipoSenha::NumerosLetras => &NL_ARR,
            TipoSenha::Especiais => &E_ARR,
            TipoSenha::Total => &T_ARR,
        };

        let c_tam:u32 = caracteres_permitidos.len() as u32; // a quantidade de caracteres no array de caracteres permitidos

        let mut senha= vec!['#';tam]; // um array de tamanho t
        // percorre cada caracter para colocar os aleatórios
        for item in senha.iter_mut(){
            // obtem um indice dos caractere aleatórios permitidos
            let indice:usize = self.rand(Intervalo::Entre(0, c_tam-1)) as usize;
            *item = caracteres_permitidos[indice] as char;
        }
        // converte para string e retorna
        senha.into_iter().collect()
    }

    // ************************************************************************************************
    // --- MÉTODOS PRIVADOS ---
    // ************************************************************************************************

    /// Retorna um número aleatório de 32 bits no intervalo total [0, u32::MAX].
    fn rand_total(&mut self) -> u32 {
        self.rng.next_u32()
    }
    
    /// Retorna um número aleatório no intervalo [a, b] inclusivo.
    fn rand_entre(&mut self, a: u32, b: u32) -> u32 {
        if a > b {
            self.rng.next_range(b..a)
        } else {
            self.rng.next_range(a..b)
        }
    }
}

/* Os arrays estáticos abaixo armazenam os intervalos de código ASCII necessários para a geração de senhas */
const N_ARR: [u8; 10] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57];
const LMAI_ARR: [u8; 26] = [65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90];
const LMIN_ARR: [u8; 26] = [97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122];
const L_ARR: [u8; 52] = [65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122];
const NL_ARR: [u8; 62] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57,65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122];
const E_ARR: [u8; 32] = [33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 58, 59, 60, 61, 62, 63, 64, 91, 92, 93, 94, 95, 96, 123, 124, 125, 126];
const T_ARR: [u8; 94] = [33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126];
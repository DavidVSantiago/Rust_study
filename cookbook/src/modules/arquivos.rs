use std::fs;
use std::str;

// ************************************************************************************************
// --- API PÚBLICA ---
// ************************************************************************************************

/// **Lê** o conteúdo de um arquivo de texto, validando-o como UTF-8.
///
/// Esta função realiza a leitura completa do arquivo do disco e tenta decodificar
/// os bytes lidos em uma String.
///
/// # Argumentos
///
/// * `nome_arquivo` - O caminho para o arquivo que será lido.
///
/// # Retorno
///
/// Retorna um `Result<String, String>` onde:
///
/// * **Ok(String):** O conteúdo completo do arquivo, garantido ser UTF-8 válido.
/// * **Err(String):** Uma mensagem de erro descritiva que ocorre se o arquivo 
///   não puder ser lido (ex: não existe, permissão) ou se o conteúdo não for 
///   uma sequência UTF-8 válida.
pub fn ler_arquivo_utf8(nome_arquivo: &str) -> Result<String, String>{
    let bytes_do_arquivo = match fs::read(nome_arquivo){
        Ok(bytes) => bytes, // se deu tudo certo, obtém os bytes
        Err(e) => return Err(format!("Erro de I/O ao tentar ler o arquivo '{}': {}", nome_arquivo, e)),
    };
    let arquivo = match String::from_utf8(bytes_do_arquivo){
        Ok(s) => s,
        Err(_) => return Err(format!("Erro ao tentar ler o arquivo '{}'. Não é UTF8 válido!", nome_arquivo)),
    };
    Ok(arquivo)
}


// ************************************************************************************************
// --- MÉTODOS PRIVADOS ---
// ************************************************************************************************
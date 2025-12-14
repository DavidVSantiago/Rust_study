
// ************************************************************************************************
// --- API PÚBLICA ---
// ************************************************************************************************

/// Conta a quantidade de **caracteres Unicode válidos** em uma string.
///
/// Esta função percorre a string usando `chars()`, o que significa que cada
/// *scalar value* Unicode é contado como um caractere, e não cada byte.
///
/// # Observação
///
/// Para UTF-8, isso garante que caracteres acentuados, símbolos e ideogramas
/// sejam corretamente contabilizados como um único elemento.
///
/// # Parâmetros
///
/// - `texto`: fatia de string a ser analisada.
///
/// # Retorno
///
/// Retorna o número total de caracteres.
pub fn conta_caracteres_texto(texto: &str) -> usize{
    texto.chars().count()
}

/// Conta a quantidade de **palavras** existentes em um texto.
///
/// Palavras são delimitadas por qualquer sequência de espaços em branco,
/// incluindo espaços, quebras de linha, tabulações e outros separadores Unicode.
///
/// # Parâmetros
///
/// - `texto`: fatia de string a ser analisada.
///
/// # Retorno
///
/// Retorna o número total de palavras encontradas.
pub fn conta_palavras_texto(texto: &str) -> usize{
    texto.split_whitespace().count()
}

/// Conta a quantidade de **linhas** em um texto.
///
/// Linhas são identificadas por caracteres separadores de linha válidos
/// para UTF-8, como `\n` e `\r\n`.
///
/// # Parâmetros
///
/// - `texto`: fatia de string a ser analisada.
///
/// # Retorno
///
/// Retorna o número total de linhas encontradas.
pub fn conta_linhas_texto(texto: &str) -> usize{
    texto.lines().count()
}

/// Conta quantas vezes uma substring ocorre dentro de um texto.
///
/// A busca pode ser sensível a maiúsculas e minúsculas ou não, conforme o
/// valor do parâmetro `ignorar_case`.
///
/// # Comportamento
///
/// - Se `ignorar_case` for `false`, a busca é feita de forma exata e sensível a caixa.
/// - Se `ignorar_case` for `true`, tanto o texto quanto o termo pesquisado são
///   convertidos para minúsculas antes da comparação.
///
/// # Parâmetros
///
/// - `texto_completo`: string onde a busca será realizada.
/// - `ocorrencia`: substring a ser procurada.
/// - `ignorar_case`: define se a comparação ignora diferenças entre maiúsculas e minúsculas.
///
/// # Retorno
///
/// Retorna o número de ocorrências encontradas.
///
/// # Observação Técnica
///
/// O uso de `to_lowercase()` cria novas strings na memória, pois a conversão
/// para minúsculas em Unicode pode alterar o tamanho do texto. Portanto,
/// essa abordagem não é *zero-copy*.
pub fn conta_ocorrencias_texto(texto_completo: &str, ocorrencia: &str, ignorar_case: bool) -> usize{
    if ignorar_case{
        texto_completo.to_lowercase().matches(&ocorrencia.to_lowercase()).count()
    }else{
        texto_completo.matches(ocorrencia).count()
    }
}
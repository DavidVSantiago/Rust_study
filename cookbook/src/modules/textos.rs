use std::str;

// ************************************************************************************************
// --- API PÚBLICA ---
// ************************************************************************************************

pub  fn conta_caracteres_texto(texto: &str) -> usize{
    texto.chars().count()
}

pub  fn conta_palavras_texto(texto: &str) -> usize{
    texto.split_whitespace().count()
}

///
pub fn conta_linhas_texto(texto: &str) -> usize{
    texto.lines().count()
}

pub fn conta_ocorrencias_texto(texto_completo: &str, ocorrencia: &str, ignorar_case: bool) -> usize{
    if ignorar_case{
        // TODO implementar isso aqui com zero-copy
        texto_completo.to_lowercase().matches(&ocorrencia.to_lowercase()).count()
    }else{
        texto_completo.matches(ocorrencia).count()
    }
}


// ************************************************************************************************
// --- MÉTODOS PRIVADOS ---
// ************************************************************************************************
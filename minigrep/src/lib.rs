pub fn pesquisar<'a>(consulta:&str,conteudo:&'a str)->Vec<&'a str>{
    // vetor que irá armazenar os resultados da busca
    let mut result = Vec::new();
    // iterar pelas linhas
    for line in conteudo.lines(){
        // verificamos se a linha atual contem o texto a ser buscado
        if line.contains(consulta){
            result.push(line); // adiciona o texto no vetor retornado
        }
    }
    result
}
pub fn pesquisar_sem_caso<'a>(consulta:&str,conteudo:&'a str)->Vec<&'a str>{
    // vetor que irá armazenar os resultados da busca
    let mut result = Vec::new();
    // iterar pelas linhas
    for line in conteudo.lines(){
        // verificamos se a linha atual contem o texto a ser buscado
        if line.to_lowercase().contains(&(consulta.to_lowercase())){
            result.push(line); // adiciona o texto no vetor retornado
        }
    }
    result
}

#[cfg(test)]
mod tests{
    use super::*; // traz a função pesquisar para este 
    
    #[test]
    fn case_sensitive(){
        let palavra_chave = "duct";
        let conteudo = "\
Rust:
safe, fast, productive.
Pick three.
Duct to give";
        assert_eq!(vec!["safe, fast, productive."],pesquisar(palavra_chave,conteudo));
    }
    #[test]
    fn case_insensitive(){
        let palavra_chave = "Rust";
        let conteudo = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],pesquisar_sem_caso(palavra_chave,conteudo));
    }
}

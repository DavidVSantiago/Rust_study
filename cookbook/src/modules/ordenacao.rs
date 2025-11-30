// ************************************************************************************************
// --- API PÚBLICA ---
// ************************************************************************************************

/// **Ordena** os elementos de um vetor de forma eficiente.
///
/// Esta função realiza a ordenação in-place de um slice mutável utilizando o
/// algoritmo de ordenação mais eficiente disponível na biblioteca padrão do Rust
/// para tipos que implementam o trait `Ord`.
///
/// A ordenação é feita diretamente sobre o array fornecido, sem alocação de memória
/// adicional proporcional ao tamanho dos dados.
///
/// # Argumentos
///
/// * `array` — um slice mutável contendo os elementos que serão ordenados.
///
/// # Detalhes de Implementação
///
/// A implementação utiliza o método `sort_unstable()`, que:
///
/// * possui complexidade média **O(n log n)**;
/// * **não preserva a ordem relativa de elementos iguais** (instável);
/// * realiza a ordenação in-place;
/// * é altamente otimizado e pode empregar variações de *introsort*
///   e algoritmos híbridos conforme o tamanho dos dados.
///
/// # Exemplos
///
/// ```
/// let mut dados = vec![3, 1, 4, 2];
/// ordena_vetor(&mut dados);
/// assert_eq!(dados, vec![1, 2, 3, 4]);
/// ```
pub fn ordena_vetor<T: Ord>(array: &mut [T]) {
    array.sort_unstable(); // invoca a função mais eficiente de ordenação em Rust
}

// ************************************************************************************************
// --- MÉTODOS PRIVADOS ---
// ************************************************************************************************

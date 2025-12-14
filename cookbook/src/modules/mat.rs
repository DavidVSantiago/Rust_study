/// **Verifica** se um número inteiro é primo.
///
/// Esta função implementa um teste de primalidade determinístico otimizado para
/// o tipo `u32`, utilizando o método de Divisão por Tentativa com a estratégia
/// "6k ± 1" (Fatoração de Roda).
///
/// A implementação é projetada para alta performance através das seguintes abordagens:
///
/// * **Casos Triviais:** Retorno imediato para 0, 1, 2, 3 e múltiplos de 2 ou 3.
/// * **Passo Otimizado:** O laço avança de 6 em 6, testando apenas candidatos da
///   forma `6k ± 1`, reduzindo o espaço de busca em aproximadamente 66%.
///
/// # Argumentos
///
/// * `num` - O número inteiro positivo (`u32`) cuja primalidade será verificada.
///
/// # Retorno
///
/// Retorna `true` se `num` for um número primo e `false` caso contrário (para números
/// compostos, 0 ou 1).
/// 
pub fn calc_primo(num: u32) -> bool{
    if num <= 3 { return num > 1; } // false p/ 1 e true p/ 2 ou 3
    if num%2 == 0 || num%3 == 0 {return false; } // falso p os multiplos de 2 ou 3
    let sup = num.isqrt();
    // Otimização 6k +/- 1
    let mut i = 5; // começa a verificar a partir de 5, pois já verificou 2 e 3
    while i <= sup{
        if num%i==0 || num%(i+2)==0 {return  false;}
        i+=6; // salta os multiplos de 2 e 3
    }
    true
}

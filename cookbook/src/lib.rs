pub mod modules; // define o módudo 'modules', torna visível (e vinculado) aqui o que está em modules/mod.rs

/* reexporta para cima. para usar: *
 * 'package::aleatorios',
 * ao invés de:
 * 'package::modules::aleatorios'*/
pub use modules::aleatorios; // importa 'aleatorios.rs' para aqui
pub use modules::arquivos; //
pub use modules::ordenacao; // importa 'ordenacao.rs' para aqui
pub use modules::tempo;
pub use modules::textos; // //
pub use modules::mat; // //

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn leitura_arquivo() {
        let result = arquivos::ler_arquivo_utf8("proibida_para_mim.txt");
        assert!(result.is_ok());
        eprintln!("{}", result.unwrap());
    }
}

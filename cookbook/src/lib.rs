pub mod modules; // define o módudo 'modules', torna visível aqui o que está em modules/mod.rs

/* reexporta para cima. para usar: *
 * 'package::aleatorios',
 * ao invés de:
 * 'package::modules::aleatorios'*/
pub use modules::aleatorios; // importa 'aleatorios.rs' para aqui
pub use modules::ordenacao; // importa 'ordenacao.rs' para aqui
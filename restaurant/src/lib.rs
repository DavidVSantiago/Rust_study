

mod front_of_house{
    pub mod hosting{ // expõe o módulo aos ancestrais
        pub fn add_lista_espera(){} // expõe a função aos ancestrais
        fn sentar_mesa(){}
    }
    mod garcon{
        fn pegar_pedido(){}
        fn servir_pedido(){}
        fn pegar_pagamento(){}
    }
}

fn comer_no_restaurante(){
    // invoca o caminho absoluto
    crate::front_of_house::hosting::add_lista_espera();

    // usamos o caminho relativo (relativo ao escopo desta função - comer_no_restaurante)
    front_of_house::hosting::add_lista_espera(); //
}
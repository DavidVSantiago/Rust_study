mod front_of_house{
    mod hosting{
        fn add_lista_espera(){}
        fn sentar_mesa(){}
    }
    mod garcon{
        fn pegar_pedido(){}
        fn servir_pedido(){}
        fn pegar_pagamento(){}
    }
}

pub fn comer_no_restaurante(){
    crate::front_of_house::hosting::add_lista_espera();

    front_of_house::hosting::add_lista_espera();
}
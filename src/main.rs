use std::io;


mod funcoes_restaurantes;
use funcoes_restaurantes::{Product, add_item, list_items, remove_item};

fn main() {
    let mut v: Vec<Product> = Vec::new();

    let mut guess = String::new();

    
    loop {
        println!("escolha sua opção!");
        println!("1 - sair | 2 - adcionar item | 3 - retirar item | 4 - listar itens");
        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        match guess.trim() {
            "1" => {  //sair
                println!("programa finalizando...");
                break;
            }
            "2" => {  //adcionar
                add_item(&mut v);
            }
            "3" => {  //retirar
                remove_item(&mut v)
            }
            "4" => {  //listar
                list_items(&v);
            }
            _ => println!("entrada inválida"),
        }
    }
}

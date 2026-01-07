mod funcoes_restaurantes;

use std::{collections::HashMap, io};
use funcoes_restaurantes::*;

fn main() {
    let mut menu: HashMap<u32, Product> = load_from_file();
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
                add_item(&mut menu);
                save_to_file(&menu);
            }
            "3" => {  //retirar
                remove_item(&mut menu);
                save_to_file(&menu);
            }
            "4" => {  //listar
                list_items(&menu);
            }
            _ => println!("entrada inválida"),
        }
    }
}
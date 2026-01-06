use std::io;


mod funcoes_restaurantes;
use funcoes_restaurantes::*;

fn main() {
    let mut v: Vec<Product> = load_from_file();

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
                save_to_file(&v);
            }
            "3" => {  //retirar
                remove_item(&mut v);
                save_to_file(&v);
            }
            "4" => {  //listar
                list_items(&v);
            }
            _ => println!("entrada inválida"),
        }
    }
}

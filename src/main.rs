use std::io;

mod funcoes_restaurantes;
use funcoes_restaurantes::*;

fn main() {
    let mut v: Vec<Product> = load_from_file(); 
    let mut guess = String::new();

    loop {
        println!("escolha sua opção!");
        println!("1 - sair | 2 - adcionar item | 3 - retirar item | 4 - listar itens");

        // Limpa o buffer para evitar que entradas anteriores interfiram na nova leitura
        guess.clear();

        io::stdin() 
            .read_line(&mut guess)
            .expect("Failed to read line");

        // O salvamento dos dados é realizada imediatamente após cada alteração (adição/remoção)
        match guess.trim() {
            "1" => { 
                println!("programa finalizando...");
                break;
            }
            "2" => { 
                add_item(&mut v);
                save_to_file(&v);
            }
            "3" => { 
                remove_item(&mut v);
                save_to_file(&v);
            }
            "4" => { 
                list_items(&v);
            }
            _ => println!("entrada inválida"),
        }
    }
}

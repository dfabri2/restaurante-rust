use std::{fs, io};
use serde::{Serialize, Deserialize};

const FILE_PATH: &str = "produtos.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    name: String,
    price: u32,
    code: u32,
}

impl Product {
pub fn create_product(name: String, price: u32, code: u32) -> Product {
        Product{name, price, code}
    }
}

pub fn save_to_file(vec: &Vec<Product>) {
    let json = serde_json::to_string_pretty(vec).expect("Erro ao serializar dados");

    fs::write(FILE_PATH, json).expect("Erro ao gravar arquivo");
}

pub fn load_from_file() -> Vec<Product> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        return serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());
    }
    Vec::new()
}

pub fn remove_item(vec: &mut Vec<Product>) {
    let mut icode: String = String::new();

    println!("insira o código do item que desejas remover");

    io::stdin().read_line(&mut icode).expect("erro ao ler código");
    let searched_code: u32 = icode.trim().parse().expect("erro ao traduzir código");

    if let Some(index) = vec.iter().position(|p|p.code == searched_code) {
        println!("removendo {}", vec[index].name);
        vec.remove(index);
    } else {
        println!("item com código {} não encontrado", searched_code);
    }
}

pub fn add_item(vec: &mut Vec<Product>) {
    println!("insira, nessa sequência, o nome, o preço e o código do produto a ser adcionado");
    
    let mut name: String = String::new();
    let mut iprice: String = String::new();
    let mut icode: String = String::new();


    io::stdin().read_line(&mut name).expect("erro ao ler nome");
    io::stdin().read_line(&mut iprice).expect("erro ao ler preço");
    io::stdin().read_line(&mut icode).expect("erro ao ler código");

    let price: u32 = iprice.trim().parse().expect("erro ao traduzir preço");
    let code: u32 = icode.trim().parse().expect("erro ao traduzir código");

    if vec.iter().any(|p|p.code == code) {
        println!("erro, já existe um item com o código informado");
    } else {
        let product: Product = Product::create_product(name.trim().to_string(), price, code);
        vec.push(product);
        println!("Produto inserido com sucesso!");
    }
}

pub fn list_items(vec: &Vec<Product>) {
    println!("---------------------");
    for i in vec {
        println!("code: {} | name: {} | price: {}", i.code, i.name, i.price);
    }
    println!("---------------------");
}
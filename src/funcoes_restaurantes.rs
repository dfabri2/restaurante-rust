use std::{collections::HashMap, fs, io};
use serde::{Serialize, Deserialize};

const FILE_PATH: &str = "produtos.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    name: String,
    price: u32,
}

impl Product {
pub fn create_product(name: String, price: u32) -> Product {
        Product{name, price}
    }
}

pub fn save_to_file(map: &HashMap<u32, Product>) {
    let json = serde_json::to_string_pretty(map).expect("Erro ao serializar dados");
    fs::write(FILE_PATH, json).expect("Erro ao gravar arquivo");
}

pub fn load_from_file() -> HashMap<u32, Product> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        return serde_json::from_str(&data).unwrap_or_else(|_| HashMap::new());
    }
    HashMap::new()
}

pub fn remove_item(map: &mut HashMap<u32, Product>) {
    let mut icode: String = String::new();

    println!("insira o código do item que desejas remover");

    io::stdin().read_line(&mut icode).expect("erro ao ler código");
    let searched_code: u32 = icode.trim().parse().expect("erro ao traduzir código");

    if map.contains_key(&searched_code) {
        println!("removendo {}", map.get(&searched_code).unwrap().name);
        map.remove(&searched_code);
    } else {
        println!("item com código {} não encontrado", searched_code);
    }
}

pub fn add_item(map: &mut HashMap<u32, Product>) {
    println!("insira, nessa sequência, o nome, o preço e o código do produto a ser adcionado");
    
    let mut name: String = String::new();
    let mut iprice: String = String::new();
    let mut icode: String = String::new();

    io::stdin().read_line(&mut name).expect("erro ao ler nome");
    io::stdin().read_line(&mut iprice).expect("erro ao ler preço");
    io::stdin().read_line(&mut icode).expect("erro ao ler código");

    let price: u32 = iprice.trim().parse().expect("erro ao traduzir preço");
    let code: u32 = icode.trim().parse().expect("erro ao traduzir código");

    if map.contains_key(&code) {
        println!("erro, já existe um item com o código informado");
    } else {
        let product: Product = Product::create_product(name.trim().to_string(), price);
        map.insert(code, product);
        println!("Produto inserido com sucesso!");
    }
}

pub fn list_items(map: &HashMap<u32, Product>) {
    if map.is_empty() {
        println!("não há produtos registrados");
    } else {
        println!("existem {} itens registrados", map.len());
        println!("");

        for (key, value) in map {
            println!("code: {} | name: {} | price: {}", key, value.name, value.price);
        }
        println!("");
    }
}
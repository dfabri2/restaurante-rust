use std::io;

pub struct Item {
    name: String,
    price: u32,
}

impl Item {
    pub fn create_item(name: String, price: u32) -> Item {
        Item {name, price}
    }
}

pub fn add_item(vec: &mut Vec<Item>) {
    let mut name: String = String::new();
    let mut price_str: String = String::new();

    println!("digite o nome do item: ");
    io::stdin()
        .read_line(&mut name)
        .expect("erro ao ler nome");

    println!("digite o preço do item: ");
    io::stdin()
        .read_line(&mut price_str)
        .expect("erro ao ler preço");

    let price: u32 = price_str.trim().parse().expect("erro ao interpretar número!");

    let new_item: Item = Item::create_item(name.trim().to_string(), price);

    vec.push(new_item);
}

pub fn list_items(vec: &Vec<Item>) {
    for i in vec {
        println!("nome: {}", i.name);
        println!("preço: {}", i.price);
    }
}
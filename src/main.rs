extern crate dc;

use dc::{Item, ItemType, Material};

fn main() {
    let item = Item::new(ItemType::named("tie"), Material::named("gold"));
    println!("I have a: {}", item);
}

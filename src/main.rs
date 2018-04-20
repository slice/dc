extern crate dc;

use dc::{Item, ItemType, Material};

fn main() {
    let item = Item::new(ItemType::Tie, Material::Gold);
    println!("I have a: {}", item.describe());
}

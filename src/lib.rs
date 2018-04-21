use std::fmt;

pub struct Item {
    pub item_type: ItemType,
    pub material: Material,
}

impl Item {
    pub fn new(item_type: ItemType, material: Material) -> Item {
        Item {
            item_type,
            material,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.material.adjective(), self.item_type.name())
    }
}

#[derive(Copy, Clone)]
pub struct ItemType(&'static str);
pub const ITEMS: [ItemType; 3] = [
    ItemType("boot"),
    ItemType("tie"),
    ItemType("helmet"),
];

impl ItemType {
    pub fn named(name: &str) -> ItemType {
        let it = ITEMS.iter().find(|&item| item.0 == name).expect("unknown item");
        (*it).clone()
    }

    pub fn name(&self) -> &'static str {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct Material(&'static str, f32);
pub const MATERIALS: [Material; 5] = [
    Material("gold", 1.5),
    Material("iron", 1.3),
    Material("copper", 1.1),

    Material("shiny", 1.8),
    Material("awesome", 2.0),
];

impl Material {
    pub fn named(name: &str) -> Material {
        let mat = MATERIALS.iter().find(|&mat| mat.0 == name).expect("unknown material");
        (*mat).clone()
    }

    pub fn adjective(&self) -> String {
        return if self.0 == "gold" {
            String::from("golden")
        } else {
            String::from(self.0)
        }
    }
}

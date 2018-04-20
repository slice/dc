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
        write!(f, "{} {}", self.material.adjective(), self.item_type.describe())
    }
}

pub enum ItemType {
    // articles of clothing
    Tie,
    Helmet,
    Shoe,

    // objects
    Plate,
    Can
}

impl ItemType {
    pub fn describe(&self) -> &str {
        match *self {
            ItemType::Tie => "tie",
            ItemType::Helmet => "helmet",
            ItemType::Shoe => "shoe",
            ItemType::Plate => "plate",
            ItemType::Can => "can",
        }
    }
}

pub enum Material {
    // basic metals
    Gold,
    Iron,
    Copper,

    // worthless materials
    Silk,
    Cobweb,
    Glass,

    // precious materials
    Shiny,
    Enhanced,
    Ultimate,
}

impl Material {
    pub fn worth(&self) -> f32 {
        match *self {
            Material::Gold => 1.5,
            Material::Iron => 1.2,
            Material::Copper => 1.2,

            Material::Silk => 0.5,
            Material::Glass => 0.7,
            Material::Cobweb => 0.2,

            Material::Shiny => 1.8,
            Material::Enhanced => 2.0,
            Material::Ultimate => 2.3,
        }
    }

    pub fn adjective(&self) -> &str {
        match *self {
            Material::Gold => "golden",
            Material::Iron => "iron",
            Material::Copper => "copper",

            Material::Silk => "silk",
            Material::Glass => "glass",
            Material::Cobweb => "web",

            Material::Shiny => "shiny",
            Material::Enhanced => "enhanced",
            Material::Ultimate => "ultimate",
        }
    }
}

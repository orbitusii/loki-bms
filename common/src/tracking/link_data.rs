#[derive(Debug, Default, Clone)]
pub struct LinkData {
    pub callsign: String,

    pub fuel: Option<f32>,

    pub weapons: Option<WeaponData>,
}

#[derive(Debug, Default, Clone)]
pub struct WeaponData {
    pub gun_ammo: u8,
    pub fox1: u8,
    pub fox2: u8,
    pub fox3: u8,
    pub others: Vec<WeaponType>,
}

#[derive(Debug, Default, Clone)]
pub struct WeaponType {
    pub name: String,
    pub quantity: u8,
}

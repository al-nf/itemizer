// src/models/stats.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_base: f64,
    pub percent_bonus: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub ability_power: Stat,
    pub armor: Stat,
    pub armor_penetration: Stat,
    pub attack_damage: Stat,
    pub attack_speed: Stat,
    pub cooldown_reduction: Stat,
    pub critical_strike_chance: Stat,
    pub gold_per_10: Stat,
    pub heal_and_shield_power: Stat,
    pub health: Stat,
    pub health_regen: Stat,
    pub lethality: Stat,
    pub lifesteal: Stat,
    pub magic_penetration: Stat,
    pub magic_resistance: Stat,
    pub mana: Stat,
    pub mana_regen: Stat,
    pub movespeed: Stat,
    pub ability_haste: Stat,
    pub omnivamp: Stat,
    pub tenacity: Stat,
}

impl Stat {
}

impl Stats {
}


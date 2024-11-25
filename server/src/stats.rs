// src/stats.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stat {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_base: f64,
    pub percent_bonus: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn new() -> Self {
        Stat {
            flat: 0.0,
            percent: 0.0,
            per_level: 0.0,
            percent_base: 0.0,
            percent_bonus: 0.0,
        }
    }
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            ability_power: Stat::new(),
            armor: Stat::new(),
            armor_penetration: Stat::new(),
            attack_damage: Stat::new(),
            attack_speed: Stat::new(),
            cooldown_reduction: Stat::new(),
            critical_strike_chance: Stat::new(),
            gold_per_10: Stat::new(),
            heal_and_shield_power: Stat::new(),
            health: Stat::new(),
            health_regen: Stat::new(),
            lethality: Stat::new(),
            lifesteal: Stat::new(),
            magic_penetration: Stat::new(),
            magic_resistance: Stat::new(),
            mana: Stat::new(),
            mana_regen: Stat::new(),
            movespeed: Stat::new(),
            ability_haste: Stat::new(),
            omnivamp: Stat::new(),
            tenacity: Stat::new(),
        }
    }
    pub fn get_stats(&self) -> Stats {
        self.clone() // Clone the stats to avoid borrowing issues in a web context
    }

    pub fn create_stats() -> Stats {
        Stats::new()
    }
}


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

    // Non-async version of create_stats
    pub fn create_stats() -> Stats {
        Stats::new()
    }

    pub fn update_stat(
        &mut self,
        stat_name: &str,
        flat: f64,
        percent: f64,
        per_level: f64,
        percent_base: f64,
        percent_bonus: f64,
    ) {
        match stat_name {
            "ability_power" => {
                self.ability_power.flat = flat;
                self.ability_power.percent = percent;
                self.ability_power.per_level = per_level;
                self.ability_power.percent_base = percent_base;
                self.ability_power.percent_bonus = percent_bonus;
            }
            "armor" => {
                self.armor.flat = flat;
                self.armor.percent = percent;
                self.armor.per_level = per_level;
                self.armor.percent_base = percent_base;
                self.armor.percent_bonus = percent_bonus;
            }
            "armor_penetration" => {
                self.armor_penetration.flat = flat;
                self.armor_penetration.percent = percent;
                self.armor_penetration.per_level = per_level;
                self.armor_penetration.percent_base = percent_base;
                self.armor_penetration.percent_bonus = percent_bonus;
            }
            "attack_damage" => {
                self.attack_damage.flat = flat;
                self.attack_damage.percent = percent;
                self.attack_damage.per_level = per_level;
                self.attack_damage.percent_base = percent_base;
                self.attack_damage.percent_bonus = percent_bonus;
            }
            "attack_speed" => {
                self.attack_speed.flat = flat;
                self.attack_speed.percent = percent;
                self.attack_speed.per_level = per_level;
                self.attack_speed.percent_base = percent_base;
                self.attack_speed.percent_bonus = percent_bonus;
            }
            "cooldown_reduction" => {
                self.cooldown_reduction.flat = flat;
                self.cooldown_reduction.percent = percent;
                self.cooldown_reduction.per_level = per_level;
                self.cooldown_reduction.percent_base = percent_base;
                self.cooldown_reduction.percent_bonus = percent_bonus;
            }
            "critical_strike_chance" => {
                self.critical_strike_chance.flat = flat;
                self.critical_strike_chance.percent = percent;
                self.critical_strike_chance.per_level = per_level;
                self.critical_strike_chance.percent_base = percent_base;
                self.critical_strike_chance.percent_bonus = percent_bonus;
            }
            "gold_per_10" => {
                self.gold_per_10.flat = flat;
                self.gold_per_10.percent = percent;
                self.gold_per_10.per_level = per_level;
                self.gold_per_10.percent_base = percent_base;
                self.gold_per_10.percent_bonus = percent_bonus;
            }
            "heal_and_shield_power" => {
                self.heal_and_shield_power.flat = flat;
                self.heal_and_shield_power.percent = percent;
                self.heal_and_shield_power.per_level = per_level;
                self.heal_and_shield_power.percent_base = percent_base;
                self.heal_and_shield_power.percent_bonus = percent_bonus;
            }
            "health" => {
                self.health.flat = flat;
                self.health.percent = percent;
                self.health.per_level = per_level;
                self.health.percent_base = percent_base;
                self.health.percent_bonus = percent_bonus;
            }
            "health_regen" => {
                self.health_regen.flat = flat;
                self.health_regen.percent = percent;
                self.health_regen.per_level = per_level;
                self.health_regen.percent_base = percent_base;
                self.health_regen.percent_bonus = percent_bonus;
            }
            "lethality" => {
                self.lethality.flat = flat;
                self.lethality.percent = percent;
                self.lethality.per_level = per_level;
                self.lethality.percent_base = percent_base;
                self.lethality.percent_bonus = percent_bonus;
            }
            "lifesteal" => {
                self.lifesteal.flat = flat;
                self.lifesteal.percent = percent;
                self.lifesteal.per_level = per_level;
                self.lifesteal.percent_base = percent_base;
                self.lifesteal.percent_bonus = percent_bonus;
            }
            "magic_penetration" => {
                self.magic_penetration.flat = flat;
                self.magic_penetration.percent = percent;
                self.magic_penetration.per_level = per_level;
                self.magic_penetration.percent_base = percent_base;
                self.magic_penetration.percent_bonus = percent_bonus;
            }
            "magic_resistance" => {
                self.magic_resistance.flat = flat;
                self.magic_resistance.percent = percent;
                self.magic_resistance.per_level = per_level;
                self.magic_resistance.percent_base = percent_base;
                self.magic_resistance.percent_bonus = percent_bonus;
            }
            "mana" => {
                self.mana.flat = flat;
                self.mana.percent = percent;
                self.mana.per_level = per_level;
                self.mana.percent_base = percent_base;
                self.mana.percent_bonus = percent_bonus;
            }
            "mana_regen" => {
                self.mana_regen.flat = flat;
                self.mana_regen.percent = percent;
                self.mana_regen.per_level = per_level;
                self.mana_regen.percent_base = percent_base;
                self.mana_regen.percent_bonus = percent_bonus;
            }
            "movespeed" => {
                self.movespeed.flat = flat;
                self.movespeed.percent = percent;
                self.movespeed.per_level = per_level;
                self.movespeed.percent_base = percent_base;
                self.movespeed.percent_bonus = percent_bonus;
            }
            "ability_haste" => {
                self.ability_haste.flat = flat;
                self.ability_haste.percent = percent;
                self.ability_haste.per_level = per_level;
                self.ability_haste.percent_base = percent_base;
                self.ability_haste.percent_bonus = percent_bonus;
            }
            "omnivamp" => {
                self.omnivamp.flat = flat;
                self.omnivamp.percent = percent;
                self.omnivamp.per_level = per_level;
                self.omnivamp.percent_base = percent_base;
                self.omnivamp.percent_bonus = percent_bonus;
            }
            "tenacity" => {
                self.tenacity.flat = flat;
                self.tenacity.percent = percent;
                self.tenacity.per_level = per_level;
                self.tenacity.percent_base = percent_base;
                self.tenacity.percent_bonus = percent_bonus;
            }
            _ => println!("Invalid stat name: {}", stat_name),
        }
    }
}


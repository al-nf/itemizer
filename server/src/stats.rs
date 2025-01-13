/*
 * File: stats.rs
 *
 * Copyright (c) 2025 ALan Fung
 *
 * Description: structs, implementations, and utility functions dealing with stats
 */
use serde::{Deserialize, Serialize};

/// stores fields of an individual statistic
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stat {
    pub flat: f64,
    pub percent: f64,
    pub per_level: f64,
    pub percent_base: f64,
    pub percent_bonus: f64,
}

/// stores all statistics as Stat structs
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

impl Default for Stat {
    fn default() -> Self {
        Self::new()
    }
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

pub fn add(stat1: &Stat, stat2: &Stat) -> Stat {
    Stat {
        flat: stat1.flat + stat2.flat,
        percent: stat1.percent + stat2.percent,
        per_level: stat1.per_level + stat2.per_level,
        percent_base: stat1.percent_base + stat2.percent_base,
        percent_bonus: stat1.percent_bonus + stat2.percent_bonus,
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
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
        self.clone() 
    }

    pub fn create_stats() -> Stats {
        Stats::new()
    }

    pub fn add_stats(stats1: &Stats, stats2: &Stats) -> Stats {
        let mut merged = Stats::new();
        
        merged.ability_power = add(&stats1.ability_power, &stats2.ability_power);
        merged.armor = add(&stats1.armor, &stats2.armor);
        merged.armor_penetration = add(&stats1.armor_penetration, &stats2.armor_penetration);
        merged.attack_damage = add(&stats1.attack_damage, &stats2.attack_damage);
        merged.attack_speed = add(&stats1.attack_speed, &stats2.attack_speed);
        merged.cooldown_reduction = add(&stats1.cooldown_reduction, &stats2.cooldown_reduction);
        merged.critical_strike_chance = add(&stats1.critical_strike_chance, &stats2.critical_strike_chance);
        merged.gold_per_10 = add(&stats1.gold_per_10, &stats2.gold_per_10);
        merged.heal_and_shield_power = add(&stats1.heal_and_shield_power, &stats2.heal_and_shield_power);
        merged.health = add(&stats1.health, &stats2.health);
        merged.health_regen = add(&stats1.health_regen, &stats2.health_regen);
        merged.lethality = add(&stats1.lethality, &stats2.lethality);
        merged.lifesteal = add(&stats1.lifesteal, &stats2.lifesteal);
        merged.magic_penetration = add(&stats1.magic_penetration, &stats2.magic_penetration);
        merged.mana = add(&stats1.mana, &stats2.mana);
        merged.mana_regen = add(&stats1.mana_regen, &stats2.mana_regen);
        merged.movespeed = add(&stats1.movespeed, &stats2.movespeed);
        merged.ability_haste = add(&stats1.ability_haste, &stats2.ability_haste);
        merged.omnivamp = add(&stats1.omnivamp, &stats2.omnivamp);
        merged.tenacity = add(&stats1.tenacity, &stats2.tenacity);

        merged
    }

}

#[derive(Debug, Default)]
pub struct StatValues {
    flat: f32,
    percent: f32,
    per_level: f32,
    percent_per_level: f32,
    percent_base: f32,
    percent_bonus: f32,
}

#[derive(Debug, Default)]
pub struct Stats {
    ability_power: StatValues,
    armor: StatValues,
    armor_penetration: StatValues,
    attack_damage: StatValues,
    attack_speed: StatValues,
    cooldown_reduction: StatValues,
    critical_strike_chance: StatValues,
    gold_per_10: StatValues,
    heal_and_shield_power: StatValues,
    health: StatValues,
    health_regen: StatValues,
    lethality: StatValues,
    lifesteal: StatValues,
    magic_penetration: StatValues,
    magic_resistance: StatValues,
    mana: StatValues,
    mana_regen: StatValues,
    movespeed: StatValues,
    ability_haste: StatValues,
    omnivamp: StatValues,
    tenacity: StatValues,
}



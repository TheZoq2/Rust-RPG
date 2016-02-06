use std::collections::HashMap;
use rand::Rng;

extern crate rand;

enum DamageTypes
{
    Blunt(f32),
    Sharp(f32),
    Magic(f32),
    Fire(f32),
    Holy(f32)
}

struct ItemTrait
{
    name: &'static str,
    modifiers: Vec<DamageTypes>
}

struct BaseWeapon
{
    name: &'static str,
    damage: Vec<DamageTypes>,
    ranged: bool
}

struct Weapon
{
    //base: BaseWeapon,
    name: String,
    damage: Vec<DamageTypes>,
}

struct GameParameters
{
    damage_traits: Vec<ItemTrait>,
    base_weapons: Vec<BaseWeapon>
}

//Returns a list of traits that can be applied to items that deal damage
fn setup_damage_traits() -> Vec<ItemTrait>
{
    let mut damage_traits: Vec<ItemTrait> = vec![];

    damage_traits.push(ItemTrait{ name: "godly", modifiers: vec![DamageTypes::Holy(2.0)]});

    damage_traits.push(ItemTrait{ name: "broken", modifiers: vec![
            DamageTypes::Blunt(0.5),
            DamageTypes::Sharp(0.5),
            DamageTypes::Magic(0.5)
        ]});

    damage_traits.push(ItemTrait{ name: "rusty", modifiers: vec![
            DamageTypes::Blunt(0.5),
            DamageTypes::Sharp(0.5),
        ]});

    damage_traits.push(ItemTrait{ name: "fine", modifiers: vec![
            DamageTypes::Blunt(1.2),
            DamageTypes::Sharp(1.2),
            DamageTypes::Magic(1.2),
        ]});

    damage_traits //return the new traits
}
fn setup_weapons() -> Vec<BaseWeapon>
{
    let mut weapons: Vec<BaseWeapon> = vec![];

    weapons.push(BaseWeapon{ name: "axe", ranged: false, damage: vec![
                DamageTypes::Blunt(0.7),
                DamageTypes::Sharp(1.2),
                DamageTypes::Magic(0.25),
                DamageTypes::Holy(0.25)
            ]}
        );
    weapons.push(BaseWeapon{ name: "shortsword", ranged: false, damage: vec![
                DamageTypes::Blunt(0.2),
                DamageTypes::Sharp(1.5),
                DamageTypes::Magic(0.25),
                DamageTypes::Holy(0.25)
            ]}
        );
    weapons.push(BaseWeapon{ name: "torch", ranged: false, damage: vec![
                DamageTypes::Blunt(0.6),
                DamageTypes::Magic(0.25),
                DamageTypes::Fire(2.0),
                DamageTypes::Holy(0.25)
            ]}
        );


    weapons
}

fn generate_weapon(parameters: GameParameters) -> Weapon
{
    let mut rng = rand::thread_rng();

    //Select a possible trait
    let weapon_trait = rng.choose(&parameters.damage_traits).unwrap();

    //Select a base
    let base = rng.choose(&parameters.base_weapons).unwrap();

    let weapon_name = weapon_trait.name.to_string() + base.name;

    println!("{}", weapon_name);

    Weapon{name: weapon_name, damage: vec![]}
}

fn main() 
{
    let damage_traits = setup_damage_traits();
    let weapons = setup_damage_traits();


    let GameParameters = 
}

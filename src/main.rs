use std::collections::HashMap;
use rand::Rng;

extern crate rand;

enum DamageTypes
{
    Blunt,
    Sharp,
    Burning,
    Poison,
    Radiation,
    END
}

struct ItemTrait
{
    name: String,
    weapon_modifiers: Vec<f32>
}
impl ItemTrait
{
    pub fn new(name: String, mods: Vec<(DamageTypes, f32)>) -> ItemTrait
    {
        let mut result = ItemTrait{name: name, weapon_modifiers: vec![1.0; DamageTypes::END as usize]};

        result.setWeaponModifiers(mods);

        result
    }

    pub fn setWeaponModifiers(&mut self, mods: Vec<(DamageTypes, f32)>)
    {
        for modifier in mods
        {
            self.weapon_modifiers[modifier.0 as usize] = modifier.1;
        }
    }
}

enum WeaponType
{
    melee,
    bullet,
    arrow,
}
struct BaseWeapon
{
    name: String,
    damage: Vec<f32>,
    weapon_type: WeaponType
}
impl BaseWeapon
{
    pub fn new(name: String, weapon_type: WeaponType, dmgs: Vec<(DamageTypes, f32)>) -> BaseWeapon
    {
        let mut result = BaseWeapon{name: name, damage: vec![0.0; DamageTypes::END as usize], weapon_type: weapon_type};

        result.setDamages(dmgs);

        result
    }

    pub fn setDamages(&mut self, dmgs: Vec<(DamageTypes, f32)>)
    {
        for dmg in dmgs
        {
            self.damage[dmg.0 as usize] = dmg.1;
        }
    }

    pub fn setDamage(&mut self, dmg_type: DamageTypes, value: f32)
    {
        self.damage[dmg_type as usize] = value;
    }
}

struct Weapon
{
    //base: BaseWeapon,
    name: String,
    damage: Vec<f32>,
}
impl Weapon
{
    pub fn new(name: String) -> Weapon
    {
        Weapon{name: name.to_string(), damage: vec![0.0; DamageTypes::END as usize]}
    }
    
    fn setDamage(&mut self, dmg_type: DamageTypes, value: f32)
    {
        self.damage[dmg_type as usize] = value;
    }
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

    //damage_traits.push(DamageTrait::new("Godly", vec![]));

    damage_traits.push(ItemTrait::new("Masterpiece".to_string(), vec![
        (DamageTypes::Sharp, 2.0),
        (DamageTypes::Blunt, 1.5),
        (DamageTypes::Burning, 1.6),
        (DamageTypes::Poison, 2.0),
        (DamageTypes::Radiation, 1.5),
    ]));

    damage_traits.push(ItemTrait::new("polished".to_string(), vec![
        (DamageTypes::Sharp, 1.5),
        (DamageTypes::Burning, 0.8),
        (DamageTypes::Poison, 1.3),
        (DamageTypes::Radiation, 1.2),
    ]));
    damage_traits.push(ItemTrait::new("broken".to_string(), vec![
        (DamageTypes::Sharp, 0.4),
        (DamageTypes::Blunt, 0.8),
    ]));
    damage_traits.push(ItemTrait::new("corroded".to_string(), vec![
        (DamageTypes::Sharp, 0.6),
        (DamageTypes::Blunt, 0.5),
        (DamageTypes::Radiation, 0.4),
        (DamageTypes::Poison, 1.2)
    ]));
    damage_traits.push(ItemTrait::new("fine".to_string(), vec![
        (DamageTypes::Sharp, 1.2),
        (DamageTypes::Blunt, 1.2),
        (DamageTypes::Radiation, 1.2),
        (DamageTypes::Poison, 1.2),
        (DamageTypes::Burning, 1.2)
    ]));
    damage_traits.push(ItemTrait::new("bent".to_string(), vec![
        (DamageTypes::Sharp, 0.8),
        (DamageTypes::Blunt, 0.9),
    ]));

    damage_traits //return the new traits
}
fn setup_weapons() -> Vec<BaseWeapon>
{
    let mut weapons: Vec<BaseWeapon> = vec![];

    weapons.push(BaseWeapon::new("axe".to_string(), WeaponType::melee, vec![
                (DamageTypes::Sharp, 1.2),
                (DamageTypes::Blunt, 1.4),
            ]));

    weapons.push(BaseWeapon::new("shortsword".to_string(), WeaponType::melee, vec![
                (DamageTypes::Sharp, 1.2),
                (DamageTypes::Blunt, 1.4),
            ]));
    weapons.push(BaseWeapon::new("dagger".to_string(), WeaponType::melee, vec![
                (DamageTypes::Sharp, 1.1),
                (DamageTypes::Poison, 1.3),
    ]));
    weapons.push(BaseWeapon::new("greatsword".to_string(), WeaponType::melee, vec![
                (DamageTypes::Sharp, 1.8),
                (DamageTypes::Blunt, 0.7)
    ]));
    weapons.push(BaseWeapon::new("pistol".to_string(), WeaponType::bullet, vec![
                (DamageTypes::Blunt, 1.0)
    ]));

    weapons
}

fn generate_weapon(parameters: GameParameters) -> Weapon
{
    const MAX_TRAITS: usize =  2;

    let mut rng = rand::thread_rng();

    //Select a possible trait
    let trait_amount = rng.gen_range(0, MAX_TRAITS + 1);
    let traits = rand::sample(&mut rng, &parameters.damage_traits, trait_amount);

    //Select a base
    let base = rng.choose(&parameters.base_weapons).unwrap();

    //Generate the name of the new weapon
    let mut weapon_name = "".to_string();
    
    for t in traits
    {
        weapon_name = weapon_name + " " + &t.name;
    }
    weapon_name = weapon_name + " " + &base.name;

    println!("{}", weapon_name);
    let mut result = Weapon::new(weapon_name);

    result
}

fn main() 
{
    let damage_traits = setup_damage_traits();
    let weapons = setup_weapons();

    let gameParams = GameParameters{damage_traits: damage_traits, base_weapons: weapons};

    generate_weapon(gameParams);

    println!("Done");
}


/* This file handles all the information about Entities aswell as any
interactions involving it. */

pub struct Entity {
    pub name: String,
    pub max_health: i32,
    pub current_health: i32,
    pub base_attack: i32,
    pub bonus_attack: i32,
}

impl Entity {
    pub fn new (name:String, max_hp:i32, base_attack:i32) -> Entity{
        Entity {
            name:name,
            max_health:max_hp,
            current_health:max_hp,
            base_attack:base_attack,
            bonus_attack:(base_attack + 15),
        }
    }

    pub fn create_enemy () -> Entity{
        Entity::new(String::from("Thug"), 100, 0)
    }

    pub fn is_dead(e: &Entity) -> bool {
        if e.current_health > 0 {false}
        else { true }
    }

    pub fn take_damage(e:&mut Entity, damage:i32) {
        if e.current_health - damage >= 0 {
            e.current_health = e.current_health - damage;
        }
        else {
            e.current_health = 0;
        }
    }

    pub fn heal(e:&mut Entity, heal:i32){
        let missing_health = e.max_health - e.current_health;
        if missing_health >= heal {
            e.current_health = e.current_health + heal;
        }
        else {
            e.current_health = e.max_health;
        }
    }
}

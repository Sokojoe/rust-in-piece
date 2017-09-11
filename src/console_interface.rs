use std::io;
use entity::Entity;

pub fn get_player_name(player_name: &mut String) {
    println!("Please enter the name of your player:");
    io::stdin().read_line(player_name)
        .expect("Failed to read line");
    player_name.pop();
    println!("Welcome to the the Wasteland: {}", player_name);
}

pub fn display_enemy(enemy:&Entity){
    println!("You have encountered a {} in the Wasteland!", enemy.name);
}

pub fn attack_entity(entity:&mut Entity, attacker: &Entity) {
    println!("{} attacks {} with {} damage.", attacker.name, entity.name, attacker.base_attack);
    Entity::take_damage(entity, attacker.base_attack);
    println!("{} has {}/{} health left!", entity.name, entity.current_health, entity.max_health);
}

pub fn display_slain(entity: &Entity){
    println!("{} has been slain.", entity.name);
}

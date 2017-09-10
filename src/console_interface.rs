use std::io;
use game::Entity;

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

pub fn attack_entity(entity:&mut Entity, damage:i32) {
    Entity::take_damage(entity, damage);
    println!("{} has {} health left!", entity.name, entity.current_health);
}

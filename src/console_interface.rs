use std::io;
use entity::Entity;
use yansi::Paint;

pub fn enable_windows_console(){
    Paint::enable_windows_ascii();
}

pub fn get_player_name(player_name: &mut String) {
    println!("Please enter the name of your player:");
    io::stdin().read_line(player_name)
        .expect("Failed to read line");
    player_name.pop();
    println!("Welcome to the the Wasteland: {}", Paint::cyan(&player_name));
}

pub fn display_enemy(enemy:&Entity){
    println!("You have encountered a {} in the Wasteland!", enemy.name);
}

pub fn attack_entity(attacker: &String, entity: &String, damage: &i32) {
    println!("{} attacks {} with {} damage.", Paint::cyan(&attacker), Paint::red(&entity), Paint::red(&damage));
}

pub fn display_health(entity: &Entity){
    println!("{} has {}/{} health left!", Paint::cyan(&entity.name), Paint::green(&entity.current_health), Paint::green(&entity.max_health));
}

pub fn display_slain(entity: &Entity){
    println!("{} {}", Paint::red(&entity.name), Paint::red("has been slain."));
}

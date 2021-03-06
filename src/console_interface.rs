use std::io;
use entity::Entity;
use yansi::Paint;
use std::collections::HashSet;

/* This file is the interface in which the player uses to interact with the
game through the terminal. Ie this file should handle all io between the player
and the game.*/

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

pub fn get_action(action: &mut String, choice: &mut HashSet<&str>) {
    println!("What action would you like to take?");
    let mut actionlist = String::new();
    for x in choice.iter() {
        actionlist.push_str(x);
        actionlist.push_str(", ");
    }
    actionlist.pop();
    actionlist.pop();
    println!("Actions: {}.", actionlist);
    io::stdin().read_line(action)
        .expect("Failed to read line");
    action.pop();
    println!();
}

pub fn display_enemy(enemy:&Entity){
    println!("You have encountered a {} in the Wasteland!", enemy.name);
}

pub fn attack_entity(attacker: &String, entity: &String, damage: &i32) {
    println!("{} attacks {} with {} damage.", Paint::cyan(&attacker), Paint::red(&entity), Paint::red(&damage));
}

pub fn display_health(entity: &Entity){
    println!("{} has {}/{} health left!", Paint::cyan(&entity.name), Paint::green(&entity.current_health), Paint::green(&entity.max_health));
    println!();
}

pub fn display_slain(entity: &Entity){
    println!("{} {}", Paint::red(&entity.name), Paint::red("has been slain."));
}

use game_interface;
use entity::Entity;
use rand::Rng;
use std::collections::HashSet;

/* This file handles all the game logic aswell as the central game loop.*/

pub fn play_game() {
    game_interface::enable_windows_console();
    let mut player_name = String::new();
    game_interface::get_player_name(&mut player_name);
    let mut player = Entity::new(player_name, 100, 5);
    loop {
        if Entity::is_dead(&player){
            break;
        }
        let roll = ::rand::thread_rng().gen_range(0, 101);
        if roll <= 20 {
            find_supplies(&mut player);
        }
        else {
            let mut enemy = Entity::create_enemy();
            combat_enemy(&mut player, &mut enemy);
            println!();
        }
    }
}

fn find_supplies(player :&mut Entity){
    println!("You find supplies and gain 20 health!");
    Entity::heal(player, 20);
    game_interface::display_health(&player);
    println!();
}

fn combat_enemy(player: &mut Entity, enemy: &mut Entity) {
    game_interface::display_enemy(&enemy);
    loop {
        let mut choice : HashSet<&str>= HashSet::new();
        choice.insert("Attack");
        choice.insert("Flee");
        let mut action = String::new();
        game_interface::get_action(&mut action, &mut choice);
        if action == "Attack".to_string(){
            let damage = ::rand::thread_rng().gen_range(player.base_attack, player.bonus_attack + 1);
            game_interface::attack_entity(&enemy.name, &player.name, &damage);
            Entity::take_damage(enemy, damage);
            game_interface::display_health(&enemy);
            if Entity::is_dead(&enemy){
                game_interface::display_slain(&enemy);
                return;
            }
            let damage = ::rand::thread_rng().gen_range(enemy.base_attack, enemy.bonus_attack + 1);
            game_interface::attack_entity(&player.name, &enemy.name, &damage);
            Entity::take_damage(player, damage);
            game_interface::display_health(&player);
            if Entity::is_dead(&player) {
                game_interface::display_slain(&player);
                return;
            }
        }
        else if action == "Flee".to_string(){
            let rn = ::rand::thread_rng().gen_range(0, 101);
            if rn >= 50{
                println!("You failed to escape.");
                let damage = ::rand::thread_rng().gen_range(enemy.base_attack, enemy.bonus_attack + 1);
                game_interface::attack_entity(&player.name, &enemy.name, &damage);
                Entity::take_damage(player, damage);
                game_interface::display_health(&player);
                if Entity::is_dead(&player) {
                    game_interface::display_slain(&player);
                    return;
                }
            }
            else {
                println!("You successfully flee the fight.");
                return;
            }
        }
        else {
            println!("Command not recognized.");
        }
    }
}

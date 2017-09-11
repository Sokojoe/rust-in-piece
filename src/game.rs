use game_interface;
use entity::Entity;

pub fn play_game() {
    let mut player_name = String::new();
    game_interface::get_player_name(&mut player_name);
    let mut player = Entity::new(player_name, 100, 5);
    loop {
        if Entity::is_dead(&player){
            break;
        }
        let mut enemy = Entity::create_enemy();
        combat_enemy(&mut player, &mut enemy);
        println!();
    }
}

fn combat_enemy(player: &mut Entity, enemy: &mut Entity) {
    game_interface::display_enemy(&enemy);
    loop {
        game_interface::attack_entity(enemy, &player);
        if Entity::is_dead(&enemy){
            game_interface::display_slain(&enemy);
            break;
        }
        game_interface::attack_entity(player, &enemy);
        if Entity::is_dead(&player) {
            game_interface::display_slain(&player);
            break;
        }
    }
}

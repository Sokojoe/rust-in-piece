use game_interface;
use entity::Entity;
use rand::Rng;

pub fn play_game() {
    game_interface::enable_windows_console();
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
        let damage = ::rand::thread_rng().gen_range(player.base_attack, player.bonus_attack + 1);
        game_interface::attack_entity(&player.name, &enemy.name, &damage);
        Entity::take_damage(enemy, damage);
        game_interface::display_health(&enemy);
        if Entity::is_dead(&enemy){
            game_interface::display_slain(&enemy);
            break;
        }
        let damage = ::rand::thread_rng().gen_range(enemy.base_attack, enemy.bonus_attack + 1);
        game_interface::attack_entity(&enemy.name, &player.name, &damage);
        Entity::take_damage(player, damage);
        game_interface::display_health(&enemy);
        if Entity::is_dead(&player) {
            game_interface::display_slain(&player);
            break;
        }
    }
}

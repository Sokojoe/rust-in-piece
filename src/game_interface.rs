use console_interface;
use game::Entity;

pub fn get_player_name(player_name:&mut String) {
    console_interface::get_player_name(player_name);
}

pub fn display_enemy(enemy:&Entity) {
    console_interface::display_enemy(enemy);
}

pub fn attack_entity(entity:&mut Entity, damage:i32) {
    console_interface::attack_entity(entity, damage);
}

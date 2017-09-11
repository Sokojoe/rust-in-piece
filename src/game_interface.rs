use console_interface;
use entity::Entity;

pub fn get_player_name(player_name:&mut String) {
    console_interface::get_player_name(player_name);
}

pub fn display_enemy(enemy:&Entity) {
    console_interface::display_enemy(enemy);
}

pub fn attack_entity(entity:&mut Entity, attacker: &Entity) {
    console_interface::attack_entity(entity, attacker);
}

pub fn display_slain(entity: &Entity){
    console_interface::display_slain(entity);
}

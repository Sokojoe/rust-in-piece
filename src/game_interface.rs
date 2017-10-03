use console_interface;
use entity::Entity;

pub fn enable_windows_console(){
    console_interface::enable_windows_console();
}

pub fn get_player_name(player_name:&mut String) {
    console_interface::get_player_name(player_name);
}

pub fn display_enemy(enemy:&Entity) {
    console_interface::display_enemy(enemy);
}

pub fn attack_entity(attacker: &String, entity: &String, damage: &i32) {
    console_interface::attack_entity(entity, attacker, damage);
}

pub fn display_health(entity: &Entity){
    console_interface::display_health(entity);
}

pub fn display_slain(entity: &Entity){
    console_interface::display_slain(entity);
}

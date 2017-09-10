use game_interface;

pub fn play_game() {
    let mut player_name = String::new();
    game_interface::get_player_name(&mut player_name);
    let mut player = Entity::new(player_name, 100, 5);
    let mut enemy = Entity::create_enemy();

    game_interface::display_enemy(&enemy);
    // testing damage function
    loop {
        if Entity::is_dead(&enemy){
            println!("You have slain enemy");
            break;
        } else if Entity::is_dead(&player) {
            println!("You have been slain by enemy");
            break;
        }
        game_interface::attack_entity(&mut enemy, player.base_attack);
        game_interface::attack_entity(&mut player, enemy.base_attack);
    }
}

impl Entity {
    pub fn new (name:String, max_health:i32, base_attack:i32) -> Entity{
        Entity {
            name:name,
            max_health:max_health,
            current_health:max_health,
            base_attack:base_attack
        }
    }

    pub fn create_enemy () -> Entity{
        Entity::new(String::from("Thug"), 20, 4)
    }

    pub fn is_dead(e: &Entity) -> bool {
        if e.current_health > 0 {false}
        else { true }
    }

    pub fn take_damage(e:&mut Entity, damage:i32) {
        if e.current_health - damage >= 0 {
            e.current_health = e.current_health - damage;
        }
        else {
            e.current_health = 0;
        }
    }
}

pub struct Entity {
    pub name: String,
    pub max_health: i32,
    pub current_health: i32,
    pub base_attack: i32,
}

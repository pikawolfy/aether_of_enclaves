use misc::*;
use piston_window::*;
use item::*;
use std::collections::HashMap;
use constants::*;

/**
    The Creature object is the template for any NPC in AOE. Primarily this is used for
    the Player, but default functionality is also implemented for Monsters and Crew.
*/

// pub enum CreatureType {
//     Player,
//     // Fighter,
//     // Cook,
//     // Carpenter,
//     // Monster,
// }

pub enum CreatureState {
    Normal,
    ControllingShip,
    // Still,
    // Moving,
    // Jumping,
    // Attacking,
    // Throwing,
    // Interacting,
}

/**
	Implementation of the Creature object.

	@field x Creature's horizontal position on screen.
	@field y Creature's vertical position on screen.
    @field self_vel_x Creature's horizontal velocity.
    @field self_vel_y Creature's vertical velocity.
    @field other_vel_x Horizontal velocity of other object(s) affecting Creature.
    @field other_vel_y Horizontal velocity of other object(s) affecting Creature.
    @field speed Creature's maximum speed when moving.
    @field health Creature's health.
    @field inventory Creature's item inventory.
    @field dir Creature's direction.
    @field sprite_index Used for animating the Creature's sprite.

*/
pub struct Creature {
    // creature_type: CreatureType,
    pub creature_state: CreatureState,
    pub x: f64,
    pub y: f64,
    self_vel_x: f64,
    self_vel_y: f64,
    directions: Vec<Direction>,
    pub other_vel_x: f64,
    pub other_vel_y: f64,
    speed: f64,
    health: i32,
    pub inventory: Option<Item>,
    dir: Direction,
    sprite_index: i32,
    frames_since_last_draw: i32,
    animation_rate: i32,
}

impl Creature {
    // Constructor for default Creature.
    pub fn new() -> Creature {
        Creature {
            // creature_type: c_type,
            creature_state: CreatureState::Normal,
            x: 0.0,
            y: 0.0,
            self_vel_x: 0.0,
            self_vel_y: 0.0,
            directions: vec![],
            other_vel_x: 0.0,
            other_vel_y: 0.0,
            speed: 2.0,
            health: 3,
            inventory: None,
            dir: Direction::S,
            sprite_index: 0,
            frames_since_last_draw: 0,
            animation_rate: 5,
        }
    }

    // Updates the position of creature based on other objects acting on it.
    pub fn update_position_other(&mut self) {
        self.x += self.other_vel_x;
        self.y += self.other_vel_y;
    }

    // Updates creature's position based on its own velocity.
    pub fn update_position_self(&mut self) {
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
        if let Some(ref mut item) = self.inventory {
            item.x = self.x;
            item.y = self.y;
        }
    }

    // Updates the direction that the creature is facing.
    pub fn update_direction(&mut self) {
        if !(self.self_vel_y == 0.0 && self.self_vel_x == 0.0) {
            if self.self_vel_x > 0.0 {
                self.dir = Direction::E;
            } else if self.self_vel_x < 0.0 {
                self.dir = Direction::W;
            }

            if self.self_vel_y > 0.0 {
                self.dir = Direction::S;
            } else if self.self_vel_y < 0.0 {
                self.dir = Direction::N;
            }
        }
    }

    // Determines where the creature is about to move.
    pub fn x_to_be_location(&self) -> f64 {
        self.x + self.self_vel_x
    }
    pub fn y_to_be_location(&self) -> f64 {
        self.y + self.self_vel_y
    }

    pub fn draw(
        &mut self,
        textures: &HashMap<String, G2dTexture>,
        context: &Context,
        graphics: &mut G2d,
        w_width: f64,
        w_height: f64,
    ) {
        for i in 0..self.health {
            image(
                textures
                    .get(IMG_HEART)
                    .expect(&format!("Not found: {:?}", IMG_HEART)),
                context
                    .transform
                    .trans(25.0 + i as f64 * (IMAGE_SIZE_SCALED + 2.0), 25.0)
                    .scale(IMAGE_SCALE, IMAGE_SCALE),
                graphics,
            );
        }

        let pic_index = self.sprite_index + 1;
        let extension;
        if self.self_vel_y != 0.0 || self.self_vel_x != 0.0 {
            extension = "player_moving_";
        } else {
            extension = "player_idle_";
        }

        let img = &format!(
            "{}{}{}{}",
            extension,
            self.dir.direction_to_string(),
            "_",
            pic_index.to_string()
        );

        image(
            textures.get(img).expect(&format!("Not found: {:?}", img)),
            context
                .transform
                .trans(w_width / 2.0, w_height / 2.0)
                .scale(IMAGE_SCALE, IMAGE_SCALE),
            graphics,
        );

        // Handle "frame rate" for animation.
        if self.frames_since_last_draw > self.animation_rate {
            self.frames_since_last_draw = 0;
            self.sprite_index = (self.sprite_index + 1) % 3;
        }
        self.frames_since_last_draw += 1;

        if let Some(_) = self.inventory {
            let item = self.inventory.clone().unwrap();
            match item.item_type {
                ItemType::Food(FoodType::Bisket) => {
                    let img = IMG_ITEM_BISKET;
                    image(
                        textures.get(img).expect(&format!("Not found: {:?}", img)),
                        context
                            .transform
                            .trans(w_width / 2.0, w_height / 2.0 - IMAGE_SIZE_SCALED * 0.7)
                            .scale(IMAGE_SCALE, IMAGE_SCALE),
                        graphics,
                    );
                }
                _ => {}
            }
        }
    }

    pub fn is_dead(&self) -> bool {
        return self.health <= 0;
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn drop_item(&mut self) -> Option<Item> {
        let temp = self.inventory.clone();
        self.inventory = None;
        temp
    }

    pub fn pickup_item(&mut self, item: Item) -> bool {
        if let None = self.inventory {
            self.inventory = Some(item);
            return true;
        }
        false
    }

    pub fn action(&mut self) {
        match self.creature_state {
            CreatureState::Normal => self.state_normal(),
            CreatureState::ControllingShip => self.state_controlling_ship(),
        }
    }

    pub fn use_item(&mut self) {
        let mut item_used = false;
        if let Some(ref item) = self.inventory {
            match item.item_type {
                ItemType::Food(_) => {
                    self.health += 1;
                    item_used = true;
                }
                _ => {}
            }
        }
        if item_used {
            self.inventory = None;
        }
    }

    fn state_controlling_ship(&mut self) {
        self.creature_state = CreatureState::Normal;
    }

    fn state_normal(&mut self) {
        self.directions = vec![];
        self.self_vel_x = 0.0;
        self.self_vel_y = 0.0;
        self.creature_state = CreatureState::ControllingShip;
    }
}

// Moving of creature.
impl Moveable for Creature {
    fn handle_input(&mut self, state: &ButtonState, key: &Option<Key>) {
        match *key {
            Some(Key::W) => {
                let dir = Direction::N;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::A) => {
                let dir = Direction::W;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::S) => {
                let dir = Direction::S;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::D) => {
                let dir = Direction::E;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            _ => {}
        }
    }

    // Updates position based on velocity.
    // Overwritten for creature.
    fn update_position(&mut self) {
        self.x += self.other_vel_x;
        self.y += self.other_vel_y;
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
    }

    // Changes the Creature's personal velocity (unrelated to other velocities acting on
    // the creature).
    // @param dx The difference in x velocity.
    // @param dy The difference in y velocity.
    fn update_self_velocity(&mut self) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        for dir in &self.directions {
            match *dir {
                Direction::N => dy -= self.speed,
                Direction::S => dy += self.speed,
                Direction::W => dx -= self.speed,
                Direction::E => dx += self.speed,
            }
        }
        self.self_vel_x = dx;
        self.self_vel_y = dy;
    }
}

use misc::*;
// use game::Game;
use piston_window::{ButtonState, Key};
use item::Item;

/**
    The Creature object is the template for any NPC in AOE. Primarily this is used for
    the Player, but default functionality is also implemented for Monsters and Crew.
*/

pub enum CreatureType {
    Player,
    // Fighter,
    // Cook,
    // Carpenter,
    // Monster,
}

pub enum CreatureState {
    Normal,
    ControllingShip,
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
*/
pub struct Creature {
    pub creature_type: CreatureType,
    pub creature_state: CreatureState,
    pub x: f64,
    pub y: f64,
    pub self_vel_x: f64,
    pub self_vel_y: f64,
    pub directions: Vec<Direction>,
    pub other_vel_x: f64,
    pub other_vel_y: f64,
    pub speed: f64,
    pub health: i32,
    pub inventory: [Option<Item>; 3],
}

impl Creature {
    // Constructor for default Creature.
    pub fn new(c_type: CreatureType) -> Creature {
        Creature {
            creature_type: c_type,
            creature_state: CreatureState::Normal,
            x: 0.0,
            y: 0.0,
            self_vel_x: 0.0,
            self_vel_y: 0.0,
            directions: vec![],
            other_vel_x: 0.0,
            other_vel_y: 0.0,
            speed: 3.0,
            health: 1,
            inventory: [None, None, None],
        }
    }

    // pub fn update(&mut self) {
    //     self.update_position();
    // }

    pub fn pickup_item(&mut self, item: Item) {
        self.inventory[0] = Some(item);
    }

    pub fn update_position_other(&mut self) {
        self.x += self.other_vel_x;
        self.y += self.other_vel_y;
    }
    pub fn update_position_self(&mut self) {
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
    }
    pub fn x_to_be_location(&mut self) -> f64 {
        self.x + self.self_vel_x
    }
    pub fn y_to_be_location(&mut self) -> f64 {
        self.y + self.self_vel_y
    }

    // TODO Write collision function.
}

impl Moveable for Creature {
    fn handle_input(&mut self, state: ButtonState, key: Option<Key>) {
        match key {
            Some(Key::W) => {
                let dir = Direction::Up;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::A) => {
                let dir = Direction::Left;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::S) => {
                let dir = Direction::Down;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::D) => {
                let dir = Direction::Right;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            _ => {}
        }
    }
    // fn collision(&mut self, game: &Game) -> bool {
    //     true
    // }

    // Updates position based on velocity.
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
                Direction::Up => dy -= self.speed,
                Direction::Down => dy += self.speed,
                Direction::Left => dx -= self.speed,
                Direction::Right => dx += self.speed,
            }
        }

        self.self_vel_x = dx;
        self.self_vel_y = dy;
    }
}

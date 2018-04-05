use piston_window::*;
use find_folder::Search;
use creature::{Creature, CreatureState, CreatureType};
use std::collections::HashMap;
use ship::Ship;
use tile::*;
use misc::*;

const WIDTH: f64 = 500.0;
const HEIGHT: f64 = 500.0;
const IMAGE_SIZE: f64 = 32.0;

#[derive(Debug, PartialEq)]

pub enum GameState {
    Title,
    InGame,
    InMenu,
}

/**
    Implementation of the Game object.

    @field input_hnd The Input Handler.
    @field player The main player.
    @field game_state The Game State (see above). 
*/
pub struct Game {
    pub player: Creature,
    pub ship: Ship,
    pub game_state: GameState,
}

impl Game {
    // Constructor of the Game.
    pub fn new() -> Self {
        let ship_tiles: Vec<Vec<i32>> = vec![
            vec![0, 0, 1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 2, 1, 1, 0],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];

        Game {
            player: Creature::new(CreatureType::Player),
            ship: Ship::new(ship_tiles),
            game_state: GameState::Title,
        }
    }

    // The function that draws stuff to the screen
    // @param e The graphics event for drawing.
    // @param window The PistonWindow that is drawn to.
    // @param textures A HashMap of texture graphics.
    fn display(
        &mut self,
        e: Event,
        window: &mut PistonWindow,
        textures: &HashMap<&str, G2dTexture>,
    ) {
        // Font locating.
        let assets = Search::ParentsThenKids(3, 3).for_folder("fonts").unwrap();
        let ref font = assets.join("Inconsolata-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

        window.draw_2d(&e, |context, graphics| {
            clear([0.0, 0.0, 0.0, 1.0], graphics); // Clears screen.
            match self.game_state {
                GameState::InGame => {
                    image(
                        textures.get("sky").unwrap(),
                        context.transform.scale(WIDTH, HEIGHT),
                        graphics,
                    );
                    for i in 0..self.ship.tiles.len() {
                        for j in 0..self.ship.tiles[i].len() {
                            match self.ship.tiles[i][j].material {
                                TileMaterial::Wood => {
                                    image(
                                        textures.get("boards").unwrap(),
                                        context.transform.trans(
                                            self.ship.x + i as f64 * IMAGE_SIZE,
                                            self.ship.y + j as f64 * IMAGE_SIZE,
                                        ),
                                        graphics,
                                    );
                                }
                                TileMaterial::Wheel => {
                                    image(
                                        textures.get("boards").unwrap(),
                                        context.transform.trans(
                                            self.ship.x + i as f64 * IMAGE_SIZE,
                                            self.ship.y + j as f64 * IMAGE_SIZE,
                                        ),
                                        graphics,
                                    );
                                    image(
                                        textures.get("wheel").unwrap(),
                                        context.transform.trans(
                                            self.ship.x + i as f64 * IMAGE_SIZE,
                                            self.ship.y + j as f64 * IMAGE_SIZE,
                                        ),
                                        graphics,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                    // Draw the player texture at player's x and y position.
                    image(
                        textures.get("mc").unwrap(),
                        context.transform.trans(
                            self.player.x,
                            self.player.y,
                        ),
                        graphics,
                    );
                }
                GameState::Title => {
                    let transform = context.transform.trans(WIDTH / 2.0, HEIGHT / 2.0);
                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16)
                        .draw(
                            "Press Enter to begin.",
                            &mut glyphs,
                            &context.draw_state,
                            transform,
                            graphics,
                        )
                        .unwrap();
                }
                GameState::InMenu => {
                    let transform = context.transform.trans(WIDTH / 2.0, HEIGHT / 2.0);
                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16)
                        .draw(
                            "This is the menu.",
                            &mut glyphs,
                            &context.draw_state,
                            transform,
                            graphics,
                        )
                        .unwrap();
                }
            }
        });
    }

    // The game loop. Displays the screen and updates events.
    // @param window The PistonWindow that is drawn to.
    // @param textures HashMap of graphics textures.
    pub fn run(&mut self, window: &mut PistonWindow, textures: HashMap<&str, G2dTexture>) {
        self.player.x = self.ship.x + ((self.ship.width / 2.0) * IMAGE_SIZE);
        self.player.y = self.ship.y + ((self.ship.height / 2.0) * IMAGE_SIZE);
        while let Some(e) = window.next() {
            match e {
                Event::Input(Input::Button(args)) => {
                    self.handle_input(
                        args.state,
                        args.button
                    );
                }

                // TODO Add lag handler here
                Event::Loop(Loop::Update(_args)) => {
                    self.player.other_vel_x = self.ship.self_vel_x;
                    self.player.other_vel_y = self.ship.self_vel_y;
                    self.player.update_position_other();
                    let x = self.player.x_to_be_location();
                    let y = self.player.y_to_be_location();
                    if self.is_on_ship(x, y) {
                        self.player.update_position_self();
                    }
                    self.ship.update_position();
                }

                Event::Loop(Loop::Render(_args)) => {
                    self.display(e, window, &textures);
                }
                _ => {}
            }
        }
    }

    // @param state The ButtonState.
    // @param button The input button arguments.
    // @param player The player.
    // @param game_state The current Game State.
    fn handle_input(&mut self, state: ButtonState, button: Button) {
        use self::Key::*;
        match button {
            Button::Keyboard(key) => match key {
                // Action button.
                Return => self.execute_action(state, None),
                // Menu toggle.
                Tab => self.execute_open_menu(state, None),
                // Move.
                W | A | S | D => self.execute_move(state, Some(key)),
                V => self.execute_change_state(state, None),
                _ => {}
            },
            _ => {}
        }
    }

    fn execute_open_menu(&mut self, state: ButtonState, _key: Option<Key>) {
        if state == ButtonState::Press {
            match self.game_state {
                GameState::InGame => {
                    println!("Menu opened.");
                    self.game_state = GameState::InMenu;
                }
                GameState::InMenu => {
                    println!("Menu closed.");
                    self.game_state = GameState::InGame;
                }
                _ => {}
            }
        }
    }

    fn execute_action(&mut self, state: ButtonState, _key: Option<Key>) {
        if state == ButtonState::Press {
            match self.game_state {
                GameState::Title => {
                    println!("Changing state to InGame.");
                    self.game_state = GameState::InGame;
                }
                _ => {}
            }
        }
    }

    fn execute_move(&mut self, state: ButtonState, key: Option<Key>) {
        match self.player.creature_state {
            CreatureState::Normal => {
                self.player.handle_input(state, key);
                self.player.update_self_velocity();
            }
            CreatureState::ControllingShip => {
                self.ship.handle_input(state, key);
                self.ship.update_self_velocity();
            }
        }
    }

    fn execute_change_state(&mut self, state: ButtonState, key: Option<Key>) {
        if state == ButtonState::Press {
            match self.player.creature_state {
                CreatureState::Normal => {
                    self.player.creature_state = CreatureState::ControllingShip;
                }
                _ => {
                    self.player.creature_state = CreatureState::Normal;
                }
            }
        }
    }

    fn is_on_ship(&mut self, x: f64, y: f64) -> bool {
        let is_in_x = x >= self.ship.x && x + IMAGE_SIZE <= self.ship.x + self.ship.width * IMAGE_SIZE;
        let is_in_y = y >= self.ship.y && y + IMAGE_SIZE <= self.ship.y + self.ship.height * IMAGE_SIZE;
        if is_in_x && is_in_y {
            let ship_tile_x = (x - self.ship.x) / IMAGE_SIZE;
            let ship_tile_y = (y - self.ship.y) / IMAGE_SIZE;
            if !self.ship.tiles[ship_tile_x.floor() as usize][ship_tile_y.floor() as usize].passable ||
            !self.ship.tiles[ship_tile_x.floor() as usize][ship_tile_y.ceil() as usize].passable ||
            !self.ship.tiles[ship_tile_x.ceil() as usize][ship_tile_y.floor() as usize].passable ||
            !self.ship.tiles[ship_tile_x.ceil() as usize][ship_tile_y.ceil() as usize].passable
            {
                return false
            }
            return true
        }
        false
    }
}

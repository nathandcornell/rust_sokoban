use ggez::{conf, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyInput;
use ggez::winit::dpi::LogicalSize;
use specs::{ RunNow, World, WorldExt};
use std::path;

mod components;
mod constants;
mod entities;
mod map;
mod resources;
mod systems;

use crate::components::*;
use crate::map::*;
use crate::resources::*;
use crate::systems::{InputSystem, GameplayStateSystem, RenderingSystem};

////
// Game Struct
////
struct Game {
    world: World,
}

impl Game {
    pub fn new(_context: &mut Context, world: World) -> Game {
        Game { world }
    }
}

////
// Event Handling
////
impl EventHandler for Game {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeat: bool
    ) -> GameResult {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(input);

        return Ok(());
    }

    fn update(&mut self, _context: &mut Context) -> GameResult {
        // Run input system:
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        // Run the game state system:
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }
        return Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut rs = RenderingSystem {context};
        rs.run_now(&self.world);

        return Ok(());
    }
}

// Render the game screen:
pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string());
}

fn main() {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    // Make a Context:
    // I'm using LogicalSize to help with scaling on different devices:
    let window_size: Option<LogicalSize<f32>> = Some(LogicalSize::new(800.0, 600.0));
    let default_window_mode = conf::WindowMode::default();
    let window_mode = conf::WindowMode {
        logical_size: window_size,
        ..default_window_mode
    };

    let (mut context, event_loop) = ContextBuilder::new("rust_sokoban", "Nate Cornell")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(window_mode)
        .add_resource_path(path::PathBuf::from("./resources"))
        .build()
        .expect("Error! Could not create game context");

    // Create an instance of an event handler.
    // In most cases, it should be provided with the Context object to use when
    // Setting up the game.
    let game = Game::new(&mut context, world);

    // Run the game
    event::run(context, event_loop, game);
}

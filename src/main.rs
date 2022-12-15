use ggez::{conf, Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use specs::{World, WorldExt};
use std::path;

mod components;

fn main() {
    let mut world = World::new();
    components::register_components(&mut world);

    // Make a Context:
    let (mut context, event_loop) = ContextBuilder::new("rust_sokoban", "Nate Cornell")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0,600.0))
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

struct Game {
    world: World,
}

impl Game {
    pub fn new(_context: &mut Context, world: World) -> Game {
        // Load/create resources like images, etc. here

        Game { world }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // TODO: Update code goes here
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, Color::WHITE);
        // TODO: Draw code goes here
        canvas.finish(context);

        return Ok(());
    }
}

use ggez::{conf, Context, ContextBuilder, event::{self, EventHandler}, GameResult, graphics::{Canvas, Color, DrawParam, Image}};
use mint::Point2;
use specs::{join::Join, ReadStorage, RunNow, System, World, WorldExt};
use std::path;

const TILE_WIDTH: f32 = 32.0;

mod components;

pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}

// System Implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (ReadStorage<'a, components::Position>, ReadStorage<'a, components::Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let mut canvas = Canvas::from_frame(self.context, Color::WHITE);
        let (positions, renderables) = data;

        // Clearing the screen (gives the bg color):
        // graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));
        // Commented out since it doesn't seem to be in ggez anymore...

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image_result = Image::from_path(self.context, renderable.path.clone());

            if image_result.is_ok() {
                let image = image_result.unwrap();
                let x = position.x as f32 * TILE_WIDTH;
                let y = position.y as f32 * TILE_WIDTH;

                // draw
                let draw_params = DrawParam::new().dest(Point2{x, y});
                canvas.draw(&image, draw_params);
            }
        }

        // Finally, present the context, this will actually display everything
        // on the screen
        canvas.finish(self.context);
    }
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
        // TODO: Draw code goes here
        let mut rs = RenderingSystem {context};
        rs.run_now(&self.world);

        return Ok(());
    }
}

// Test entities to render:
pub fn initialize_level(world: &mut World) {
    components::create_player(
        world,
        components::Position {
            x: 0,
            y: 0,
            z: 0,
        }
    );
    components::create_wall(
        world,
        components::Position {
            x: 1,
            y: 0,
            z: 0,
        }
    );
    components::create_box(
        world,
        components::Position {
            x: 2,
            y: 0,
            z: 0,
        }
    );
}

fn main() {
    let mut world = World::new();
    components::register_components(&mut world);
    initialize_level(&mut world);

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

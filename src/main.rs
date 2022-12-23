use ggez::{conf, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::graphics::{Canvas, Color, DrawParam, Image};
use ggez::winit::dpi::{LogicalSize};
use mint::Point2;
use specs::{
    Entities, ReadStorage, RunNow, System, World, WorldExt, Write, WriteStorage
};
use specs::world::Index;
use specs::join::Join;
use std::path;
use std::collections::HashMap;

const TILE_WIDTH: f32 = 32.0;
const MAP_HEIGHT: u8 = 8;
const MAP_WIDTH: u8 = 9;

mod components;

////
// Resources:
////
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyInput>,
}

// Register resources to world:
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}

////
// Systems
////

// Rendering System
pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}

// Rendering System Implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (ReadStorage<'a, components::Position>, ReadStorage<'a, components::Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let scale_factor: f64 = self.context.gfx.window().scale_factor();
        let mut canvas = Canvas::from_frame(self.context, Color::BLACK);
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
                let x = position.x as f32 * TILE_WIDTH * scale_factor as f32;
                let y = position.y as f32 * TILE_WIDTH * scale_factor as f32;

                // draw
                let draw_params = DrawParam::new().dest(Point2{x, y})
                    .scale(Point2{x: scale_factor as f32, y: scale_factor as f32});
                canvas.draw(&image, draw_params);
            }
        }

        // Finally, present the context, this will actually display everything
        // on the screen
        canvas.finish(self.context);
    }
}

// Input system
pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, components::Position>,
        ReadStorage<'a, components::Player>,
        ReadStorage<'a, components::Moveable>,
        ReadStorage<'a, components::Immoveable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, mut positions, players, moveables, immoveables) = data;

        let mut to_move = Vec::new();

        for (position, _player) in (&positions, &players).join() {
            // Get the first keypress
            if let Some(key) = input_queue.keys_pressed.pop() {
                // Get the moveables and immoveables:
                let mov: HashMap<(u8, u8), Index> = (&entities, &moveables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                let immov: HashMap<(u8, u8), Index> = (&entities, &immoveables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                // Now we'll check everything from the current position through
                // this axis to see what can and must move
                let (start, end, is_x) = match key.keycode.unwrap() {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    // Find a moveable
                    // If it exists, try to move it and continue
                    // If it doesn't exist, we continue and look for an
                    // immoveable instead
                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => {
                            // Find an immoveable. 
                            // If it exists, we'll stop and move nothing
                            // If not, we stop because we found a gap (floor)
                            match immov.get(&pos) {
                                Some(_id) => to_move.clear(),
                                None => break,
                            }
                        }
                    }
                }

            }
        }

        // Now move everything that can and must be moved
        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key.keycode.unwrap() {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}

////
// Game Struct
////
struct Game {
    world: World,
}

impl Game {
    pub fn new(_context: &mut Context, world: World) -> Game {
        // Load/create resources like images, etc. here

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
        return Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // TODO: Draw code goes here
        let mut rs = RenderingSystem {context};
        rs.run_now(&self.world);

        return Ok(());
    }
}

// Map loader
pub fn load_map(world: &mut World, map_string: String) {
    // Read each line into a Vector:
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            // Calculate the position on the map where this entity will be drawn
            let position = components::Position {
                x: x as u8,
                y: y as u8,
                z: 0, // we'll override this with the entity z value
            };

            // Create objects according to our key value:
            match *column {
                "." => components::create_floor(world, position),
                "W" => {
                    components::create_floor(world, position);
                    components::create_wall(world, position);
                }
                "P" => {
                    components::create_floor(world, position);
                    components::create_player(world, position);
                }
                "B" => {
                    components::create_floor(world, position);
                    components::create_box(world, position);
                }
                "S" => {
                    components::create_floor(world, position);
                    components::create_box_spot(world, position);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
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
    components::register_components(&mut world);
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

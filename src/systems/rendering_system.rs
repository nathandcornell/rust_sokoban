////
// Rendering System
////

use ggez::Context;
use ggez::graphics::{Canvas, Color, DrawParam, Image, PxScale, Text};
use mint::Point2;
use specs::{Read, ReadStorage, System};
use specs::join::Join;

use crate::components::*;
use crate::constants::TILE_WIDTH;
use crate::resources::Gameplay;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

// Drawing text
impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32, canvas: &mut Canvas) {
        let scale_factor: f64 = self.context.gfx.window().scale_factor();
        let text = Text::new(text_string);
        let destination = Point2{x, y};
        let color = Some(Color::new(255.0, 255.0, 255.0, 0.9));

        let draw_params = DrawParam::new()
            .color(color.unwrap())
            .dest(destination)
            .scale(Point2{x: scale_factor as f32, y: scale_factor as f32});
        &canvas.draw(&text, draw_params);
    }
}

// Rendering System Implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (
        Read<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>
    );

    fn run(&mut self, data: Self::SystemData) {
        let scale_factor: f64 = self.context.gfx.window().scale_factor();
        let mut canvas = Canvas::from_frame(self.context, Color::BLACK);
        let (gameplay, positions, renderables) = data;

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

        // Draw state and move count:
        self.draw_text(&gameplay.state.to_string(), 600.0, 100.0, &mut canvas);
        self.draw_text(&gameplay.moves_count.to_string(), 600.0, 150.0, &mut canvas);

        // Finally, present the context, this will actually display everything
        // on the screen
        canvas.finish(self.context);
    }
}

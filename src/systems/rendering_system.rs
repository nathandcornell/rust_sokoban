////
// Rendering System
////

use ggez::Context;
use ggez::graphics::{Canvas, Color, DrawParam, Image};
use mint::Point2;
use specs::{ReadStorage, System};
use specs::join::Join;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

use crate::components::*;
use crate::constants::TILE_WIDTH;

// Rendering System Implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let scale_factor: f64 = self.context.gfx.window().scale_factor();
        let mut canvas = Canvas::from_frame(self.context, Color::BLACK);
        let (positions, renderables) = data;

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

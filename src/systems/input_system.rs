////
// Input system
////
use ggez::input::keyboard::KeyCode;
use specs::{Entities, ReadStorage, System, Write, WriteStorage};
use specs::join::Join;
use specs::world::Index;
use std::collections::HashMap;

use crate::components::*;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::resources::InputQueue;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Moveable>,
        ReadStorage<'a, Immoveable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, mut positions, players, moveables, immoveables) = data;

        let mut to_move = Vec::new();

        // TODO: Refactor this; it's an n*m that runs after every keypress.
        //       The more complex a level is, the slower gameplay will be.
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

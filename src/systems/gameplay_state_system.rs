////
// Gameplay State System
////

use specs::{Join, ReadStorage, System, Write};
use std::collections::HashMap;

use crate::{
    components::{Box, BoxSpot, Position},
    resources::{Gameplay, GameplayState}
};

pub struct GameplayStateSystem{}

impl<'a> System<'a> for GameplayStateSystem {
    type SystemData = (
        Write<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>
    );

    fn run(&mut self, data: Self::SystemData) {
        let(mut gameplay_state, positions, boxes, box_spots) = data;

        // Bail early if the game is already won:
        if gameplay_state.state == GameplayState::Won { return; }

        // Get boxes, indexed by their positions
        let boxes_by_position: HashMap<(u8, u8), &Box> = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();

        // Loop through box spots to see if there's a box at the same position
        // If not, bail.
        for (_box_spot, position) in (&box_spots, &positions).join() {
            if !boxes_by_position.contains_key(&(position.x, position.y)) {
                return; // Quit this function
            }
        }

        // If we made it this far, all the spots matched with a box.
        // Change the game state to indicate success!
        gameplay_state.state = GameplayState::Won;
        println!("Won the game in {} moves!", gameplay_state.moves_count);
    }
}

////
// Resources:
////
use ggez::input::keyboard::KeyInput;
use specs::World;

#[derive(Eq, Debug, PartialEq)]
pub enum GameplayState {
    Playing,
    Won
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyInput>,
}

// Register resources to world:
pub fn register_resources(world: &mut World) {
    world.insert(Gameplay::default());
    world.insert(InputQueue::default());
}

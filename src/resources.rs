////
// Resources:
////
use ggez::input::keyboard::KeyInput;
use specs::World;
use std::fmt;

#[derive(Eq, PartialEq)]
pub enum GameplayState {
    Playing,
    Won
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl fmt::Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won!"
        })?;
        Ok(())
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

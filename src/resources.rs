////
// Resources:
////
use ggez::input::keyboard::KeyInput;
use specs::World;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyInput>,
}

// Register resources to world:
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}

use specs::{Component, NullStorage, VecStorage, World, WorldExt};

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct BoxSpot {
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Moveable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immoveable;

pub fn register_components(world: &mut World) {
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Immoveable>();
    world.register::<Moveable>();
    world.register::<Player>();
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Wall>();
}

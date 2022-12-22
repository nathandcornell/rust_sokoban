use specs::{Builder, Component, NullStorage, VecStorage, World, WorldExt};

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

#[derive(Component)]
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

pub fn create_box(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/box.png".to_string(),
        })
        .with(Moveable)
        .with(Box {})
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable {
            path: "/images/box_spot.png".to_string(),
        })
        .with(Player {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable {
            path: "/images/floor.png".to_string(),
        })
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/player.png".to_string(),
        })
        .with(Moveable)
        .with(Player {})
        .build();
}

pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/wall.png".to_string(),
        })
        .with(Immoveable)
        .with(Wall {})
        .build();
}
